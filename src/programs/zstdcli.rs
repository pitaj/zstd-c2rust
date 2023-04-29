use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type FIO_ctx_s;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut g_utilDisplayLevel: libc::c_int;
    fn UTIL_isLink(infilename: *const libc::c_char) -> libc::c_int;
    fn UTIL_isFIFO(infilename: *const libc::c_char) -> libc::c_int;
    fn UTIL_isConsole(file: *mut FILE) -> libc::c_int;
    fn UTIL_fakeStdinIsConsole();
    fn UTIL_fakeStdoutIsConsole();
    fn UTIL_fakeStderrIsConsole();
    fn UTIL_traceFileStat();
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    fn UTIL_createFileNamesTable_fromFileName(
        inputFileName: *const libc::c_char,
    ) -> *mut FileNamesTable;
    fn UTIL_freeFileNamesTable(table: *mut FileNamesTable);
    fn UTIL_mergeFileNamesTable(
        table1: *mut FileNamesTable,
        table2: *mut FileNamesTable,
    ) -> *mut FileNamesTable;
    fn UTIL_expandFNT(fnt: *mut *mut FileNamesTable, followLinks: libc::c_int);
    fn UTIL_allocateFileNamesTable(tableSize: size_t) -> *mut FileNamesTable;
    fn UTIL_searchFileNamesTable(
        table: *mut FileNamesTable,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn UTIL_refFilename(fnt: *mut FileNamesTable, filename: *const libc::c_char);
    fn ZSTD_versionString() -> *const libc::c_char;
    fn ZSTD_minCLevel() -> libc::c_int;
    fn ZSTD_maxCLevel() -> libc::c_int;
    fn ZSTD_cParam_getBounds(cParam: ZSTD_cParameter) -> ZSTD_bounds;
    fn ZSTD_getCParams(
        compressionLevel: libc::c_int,
        estimatedSrcSize: libc::c_ulonglong,
        dictSize: size_t,
    ) -> ZSTD_compressionParameters;
    fn FIO_setLdmHashRateLog(prefs: *mut FIO_prefs_t, ldmHashRateLog: libc::c_int);
    fn FIO_createPreferences() -> *mut FIO_prefs_t;
    fn FIO_freePreferences(prefs: *mut FIO_prefs_t);
    fn FIO_createContext() -> *mut FIO_ctx_t;
    fn FIO_freeContext(fCtx: *mut FIO_ctx_t);
    fn FIO_setCompressionType(
        prefs: *mut FIO_prefs_t,
        compressionType: FIO_compressionType_t,
    );
    fn FIO_overwriteMode(prefs: *mut FIO_prefs_t);
    fn FIO_setAdaptiveMode(prefs: *mut FIO_prefs_t, adapt: libc::c_int);
    fn FIO_setAdaptMin(prefs: *mut FIO_prefs_t, minCLevel: libc::c_int);
    fn FIO_setAdaptMax(prefs: *mut FIO_prefs_t, maxCLevel: libc::c_int);
    fn FIO_setUseRowMatchFinder(prefs: *mut FIO_prefs_t, useRowMatchFinder: libc::c_int);
    fn FIO_setBlockSize(prefs: *mut FIO_prefs_t, blockSize: libc::c_int);
    fn FIO_setChecksumFlag(prefs: *mut FIO_prefs_t, checksumFlag: libc::c_int);
    fn FIO_setDictIDFlag(prefs: *mut FIO_prefs_t, dictIDFlag: libc::c_int);
    fn FIO_setLdmBucketSizeLog(prefs: *mut FIO_prefs_t, ldmBucketSizeLog: libc::c_int);
    fn FIO_setLdmFlag(prefs: *mut FIO_prefs_t, ldmFlag: libc::c_uint);
    fn FIO_setHasStdoutOutput(fCtx: *mut FIO_ctx_t, value: libc::c_int);
    fn FIO_setAllowBlockDevices(prefs: *mut FIO_prefs_t, allowBlockDevices: libc::c_int);
    fn FIO_setPatchFromMode(prefs: *mut FIO_prefs_t, value: libc::c_int);
    fn FIO_setContentSize(prefs: *mut FIO_prefs_t, value: libc::c_int);
    fn FIO_displayCompressionParameters(prefs: *const FIO_prefs_t);
    fn FIO_setAsyncIOFlag(prefs: *mut FIO_prefs_t, value: libc::c_int);
    fn FIO_setPassThroughFlag(prefs: *mut FIO_prefs_t, value: libc::c_int);
    fn FIO_setMMapDict(prefs: *mut FIO_prefs_t, value: ZSTD_paramSwitch_e);
    fn FIO_setNbFilesTotal(fCtx: *mut FIO_ctx_t, value: libc::c_int);
    fn FIO_setExcludeCompressedFile(
        prefs: *mut FIO_prefs_t,
        excludeCompressedFiles: libc::c_int,
    );
    fn FIO_setLdmHashLog(prefs: *mut FIO_prefs_t, ldmHashLog: libc::c_int);
    fn FIO_setLdmMinMatch(prefs: *mut FIO_prefs_t, ldmMinMatch: libc::c_int);
    fn FIO_setMemLimit(prefs: *mut FIO_prefs_t, memLimit: libc::c_uint);
    fn FIO_setNbWorkers(prefs: *mut FIO_prefs_t, nbWorkers: libc::c_int);
    fn FIO_setOverlapLog(prefs: *mut FIO_prefs_t, overlapLog: libc::c_int);
    fn FIO_setRemoveSrcFile(prefs: *mut FIO_prefs_t, flag: libc::c_int);
    fn FIO_setSparseWrite(prefs: *mut FIO_prefs_t, sparse: libc::c_int);
    fn FIO_setRsyncable(prefs: *mut FIO_prefs_t, rsyncable: libc::c_int);
    fn FIO_setStreamSrcSize(prefs: *mut FIO_prefs_t, streamSrcSize: size_t);
    fn FIO_setTargetCBlockSize(prefs: *mut FIO_prefs_t, targetCBlockSize: size_t);
    fn FIO_setSrcSizeHint(prefs: *mut FIO_prefs_t, srcSizeHint: size_t);
    fn FIO_setNotificationLevel(level: libc::c_int);
    fn FIO_setProgressSetting(progressSetting: FIO_progressSetting_e);
    fn FIO_setLiteralCompressionMode(prefs: *mut FIO_prefs_t, mode: ZSTD_paramSwitch_e);
    fn FIO_setTestMode(prefs: *mut FIO_prefs_t, testMode: libc::c_int);
    fn FIO_determineHasStdinInput(
        fCtx: *mut FIO_ctx_t,
        filenames: *const FileNamesTable,
    );
    fn FIO_compressFilename(
        fCtx: *mut FIO_ctx_t,
        prefs: *mut FIO_prefs_t,
        outfilename: *const libc::c_char,
        infilename: *const libc::c_char,
        dictFileName: *const libc::c_char,
        compressionLevel: libc::c_int,
        comprParams: ZSTD_compressionParameters,
    ) -> libc::c_int;
    fn FIO_decompressFilename(
        fCtx: *mut FIO_ctx_t,
        prefs: *mut FIO_prefs_t,
        outfilename: *const libc::c_char,
        infilename: *const libc::c_char,
        dictFileName: *const libc::c_char,
    ) -> libc::c_int;
    fn FIO_listMultipleFiles(
        numFiles: libc::c_uint,
        filenameTable: *mut *const libc::c_char,
        displayLevel: libc::c_int,
    ) -> libc::c_int;
    fn FIO_compressMultipleFilenames(
        fCtx: *mut FIO_ctx_t,
        prefs: *mut FIO_prefs_t,
        inFileNamesTable: *mut *const libc::c_char,
        outMirroredDirName: *const libc::c_char,
        outDirName: *const libc::c_char,
        outFileName: *const libc::c_char,
        suffix: *const libc::c_char,
        dictFileName: *const libc::c_char,
        compressionLevel: libc::c_int,
        comprParams: ZSTD_compressionParameters,
    ) -> libc::c_int;
    fn FIO_decompressMultipleFilenames(
        fCtx: *mut FIO_ctx_t,
        prefs: *mut FIO_prefs_t,
        srcNamesTable: *mut *const libc::c_char,
        outMirroredDirName: *const libc::c_char,
        outDirName: *const libc::c_char,
        outFileName: *const libc::c_char,
        dictFileName: *const libc::c_char,
    ) -> libc::c_int;
    fn FIO_addAbortHandler();
    fn FIO_zlibVersion() -> *const libc::c_char;
    fn FIO_lz4Version() -> *const libc::c_char;
    fn FIO_lzmaVersion() -> *const libc::c_char;
    fn AIO_supported() -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const libc::c_char,
    pub buf: *mut libc::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type ZSTD_cParameter = libc::c_uint;
pub const ZSTD_c_experimentalParam19: ZSTD_cParameter = 1016;
pub const ZSTD_c_experimentalParam18: ZSTD_cParameter = 1015;
pub const ZSTD_c_experimentalParam17: ZSTD_cParameter = 1014;
pub const ZSTD_c_experimentalParam16: ZSTD_cParameter = 1013;
pub const ZSTD_c_experimentalParam15: ZSTD_cParameter = 1012;
pub const ZSTD_c_experimentalParam14: ZSTD_cParameter = 1011;
pub const ZSTD_c_experimentalParam13: ZSTD_cParameter = 1010;
pub const ZSTD_c_experimentalParam12: ZSTD_cParameter = 1009;
pub const ZSTD_c_experimentalParam11: ZSTD_cParameter = 1008;
pub const ZSTD_c_experimentalParam10: ZSTD_cParameter = 1007;
pub const ZSTD_c_experimentalParam9: ZSTD_cParameter = 1006;
pub const ZSTD_c_experimentalParam8: ZSTD_cParameter = 1005;
pub const ZSTD_c_experimentalParam7: ZSTD_cParameter = 1004;
pub const ZSTD_c_experimentalParam6: ZSTD_cParameter = 1003;
pub const ZSTD_c_experimentalParam5: ZSTD_cParameter = 1002;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: libc::c_int,
    pub upperBound: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
pub type ZSTD_paramSwitch_e = libc::c_uint;
pub const ZSTD_ps_disable: ZSTD_paramSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_paramSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_paramSwitch_e = 0;
pub type FIO_progressSetting_e = libc::c_uint;
pub const FIO_ps_always: FIO_progressSetting_e = 2;
pub const FIO_ps_never: FIO_progressSetting_e = 1;
pub const FIO_ps_auto: FIO_progressSetting_e = 0;
pub type FIO_compressionType_t = libc::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_prefs_s {
    pub compressionType: FIO_compressionType_t,
    pub sparseFileSupport: libc::c_int,
    pub dictIDFlag: libc::c_int,
    pub checksumFlag: libc::c_int,
    pub blockSize: libc::c_int,
    pub overlapLog: libc::c_int,
    pub adaptiveMode: libc::c_int,
    pub useRowMatchFinder: libc::c_int,
    pub rsyncable: libc::c_int,
    pub minAdaptLevel: libc::c_int,
    pub maxAdaptLevel: libc::c_int,
    pub ldmFlag: libc::c_int,
    pub ldmHashLog: libc::c_int,
    pub ldmMinMatch: libc::c_int,
    pub ldmBucketSizeLog: libc::c_int,
    pub ldmHashRateLog: libc::c_int,
    pub streamSrcSize: size_t,
    pub targetCBlockSize: size_t,
    pub srcSizeHint: libc::c_int,
    pub testMode: libc::c_int,
    pub literalCompressionMode: ZSTD_paramSwitch_e,
    pub removeSrcFile: libc::c_int,
    pub overwrite: libc::c_int,
    pub asyncIO: libc::c_int,
    pub memLimit: libc::c_uint,
    pub nbWorkers: libc::c_int,
    pub excludeCompressedFiles: libc::c_int,
    pub patchFromMode: libc::c_int,
    pub contentSize: libc::c_int,
    pub allowBlockDevices: libc::c_int,
    pub passThrough: libc::c_int,
    pub mmapDict: ZSTD_paramSwitch_e,
}
pub type FIO_prefs_t = FIO_prefs_s;
pub type FIO_ctx_t = FIO_ctx_s;
pub type zstd_operation_mode = libc::c_uint;
pub const zom_list: zstd_operation_mode = 5;
pub const zom_train: zstd_operation_mode = 4;
pub const zom_bench: zstd_operation_mode = 3;
pub const zom_test: zstd_operation_mode = 2;
pub const zom_decompress: zstd_operation_mode = 1;
pub const zom_compress: zstd_operation_mode = 0;
pub const ZSTDCLI_CLEVEL_DEFAULT: libc::c_int = 3 as libc::c_int;
pub const ZSTDCLI_CLEVEL_MAX: libc::c_int = 19 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const UTIL_FILESIZE_UNKNOWN: libc::c_int = -(1 as libc::c_int);
pub const ZSTD_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".zst\0")
};
pub const stdoutmark: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"/*stdout*\\\0")
};
pub const stdinmark: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/*stdin*\\\0")
};
pub const nulmark: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/dev/null\0")
};
pub const LZ4_EXTENSION: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".lz4\0")
};
pub const XZ_EXTENSION: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".xz\0")
};
pub const LZMA_EXTENSION: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b".lzma\0")
};
pub const GZ_EXTENSION: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".gz\0")
};
pub const ZSTD_ZSTDMT: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"zstdmt\0")
};
pub const ZSTD_UNZSTD: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"unzstd\0")
};
pub const ZSTD_CAT: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"zstdcat\0")
};
pub const ZSTD_ZCAT: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"zcat\0")
};
pub const ZSTD_GZ: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"gzip\0")
};
pub const ZSTD_GUNZIP: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"gunzip\0")
};
pub const ZSTD_GZCAT: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"gzcat\0")
};
pub const ZSTD_LZMA: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"lzma\0")
};
pub const ZSTD_UNLZMA: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"unlzma\0")
};
pub const ZSTD_XZ: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"xz\0")
};
pub const ZSTD_UNXZ: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"unxz\0")
};
pub const ZSTD_LZ4: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"lz4\0")
};
pub const ZSTD_UNLZ4: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"unlz4\0")
};
pub const DISPLAY_LEVEL_DEFAULT: libc::c_int = 2 as libc::c_int;
static mut g_defaultDictName: *const libc::c_char = b"dictionary\0" as *const u8
    as *const libc::c_char;
static mut g_defaultMaxDictSize: libc::c_uint = (110 as libc::c_int
    * ((1 as libc::c_int) << 10 as libc::c_int)) as libc::c_uint;
static mut g_defaultDictCLevel: libc::c_int = 3 as libc::c_int;
static mut g_defaultSelectivityLevel: libc::c_uint = 9 as libc::c_int as libc::c_uint;
static mut g_defaultMaxWindowLog: libc::c_uint = 27 as libc::c_int as libc::c_uint;
pub const OVERLAP_LOG_DEFAULT: libc::c_int = 9999 as libc::c_int;
pub const LDM_PARAM_DEFAULT: libc::c_int = 9999 as libc::c_int;
static mut g_overlapLog: U32 = OVERLAP_LOG_DEFAULT as U32;
static mut g_ldmHashLog: U32 = 0 as libc::c_int as U32;
static mut g_ldmMinMatch: U32 = 0 as libc::c_int as U32;
static mut g_ldmHashRateLog: U32 = LDM_PARAM_DEFAULT as U32;
static mut g_ldmBucketSizeLog: U32 = LDM_PARAM_DEFAULT as U32;
static mut g_displayLevel: libc::c_int = DISPLAY_LEVEL_DEFAULT;
unsafe extern "C" fn checkLibVersion() {
    if strcmp(b"1.5.5\0" as *const u8 as *const libc::c_char, ZSTD_versionString()) != 0
    {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : incorrect library version (expecting : %s ; actual : %s ) \n\0"
                    as *const u8 as *const libc::c_char,
                b"1.5.5\0" as *const u8 as *const libc::c_char,
                ZSTD_versionString(),
            );
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Please update library to version %s, or use stand-alone zstd binary \n\0"
                    as *const u8 as *const libc::c_char,
                b"1.5.5\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn exeNameMatch(
    mut exeName: *const libc::c_char,
    mut test: *const libc::c_char,
) -> libc::c_int {
    return (strncmp(exeName, test, strlen(test)) == 0
        && (*exeName.offset(strlen(test) as isize) as libc::c_int == '\0' as i32
            || *exeName.offset(strlen(test) as isize) as libc::c_int == '.' as i32))
        as libc::c_int;
}
unsafe extern "C" fn usage(mut f: *mut FILE, mut programName: *const libc::c_char) {
    fprintf(
        f,
        b"Compress or decompress the INPUT file(s); reads from STDIN if INPUT is `-` or not provided.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"Usage: %s [OPTIONS...] [INPUT... | -] [-o OUTPUT]\n\n\0" as *const u8
            as *const libc::c_char,
        programName,
    );
    fprintf(f, b"Options:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        f,
        b"  -o OUTPUT                     Write output to a single file, OUTPUT.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -k, --keep                    Preserve INPUT file(s). [Default] \n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"  --rm                          Remove INPUT file(s) after successful (de)compression.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        f,
        b"  -#                            Desired compression level, where `#` is a number between 1 and %d;\n\0"
            as *const u8 as *const libc::c_char,
        19 as libc::c_int,
    );
    fprintf(
        f,
        b"                                lower numbers provide faster compression, higher numbers yield\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"                                better compression ratios. [Default: %d]\n\n\0"
            as *const u8 as *const libc::c_char,
        3 as libc::c_int,
    );
    fprintf(
        f,
        b"  -d, --decompress              Perform decompression.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -D DICT                       Use DICT as the dictionary for compression or decompression.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -f, --force                   Disable input and output checks. Allows overwriting existing files,\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"                                receiving input from the console, printing output to STDOUT, and\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"                                operating on links, block devices, etc. Unrecognized formats will be\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"                                passed-through through as-is.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -h                            Display short usage and exit.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -H, --help                    Display full help and exit.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"  -V, --version                 Display the program version and exit.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn usage_advanced(mut programName: *const libc::c_char) {
    fprintf(
        stdout,
        b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8 as *const libc::c_char,
        b"Zstandard CLI\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"v1.5.5\0" as *const u8 as *const libc::c_char,
        b"Yann Collet\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    usage(stdout, programName);
    fprintf(stdout, b"Advanced options:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"  -c, --stdout                  Write to STDOUT (even if it is a console) and keep the INPUT file(s).\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  -v, --verbose                 Enable verbose output; pass multiple times to increase verbosity.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  -q, --quiet                   Suppress warnings; pass twice to suppress errors.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"  --[no-]progress               Forcibly show/hide the progress counter. NOTE: Any (de)compressed\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"                                output to terminal will mix with progress counter text.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  -r                            Operate recursively on directories.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --filelist LIST               Read a list of files to operate on from LIST.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --output-dir-flat DIR         Store processed files in DIR.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --output-dir-mirror DIR       Store processed files in DIR, respecting original directory structure.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if AIO_supported() != 0 {
        fprintf(
            stdout,
            b"  --[no-]asyncio                Use asynchronous IO. [Default: Enabled]\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"  --[no-]check                  Add XXH64 integrity checksums during compression. [Default: Add, Validate]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"                                If `-d` is present, ignore/validate checksums during decompression.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"  --                            Treat remaining arguments after `--` as files.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"Advanced compression options:\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --ultra                       Enable levels beyond %i, up to %i; requires more memory.\n\0"
            as *const u8 as *const libc::c_char,
        19 as libc::c_int,
        ZSTD_maxCLevel(),
    );
    fprintf(
        stdout,
        b"  --fast[=#]                    Use to very fast compression levels. [Default: %u]\n\0"
            as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    fprintf(
        stdout,
        b"  --adapt                       Dynamically adapt compression level to I/O conditions.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --long[=#]                    Enable long distance matching with window log #. [Default: %u]\n\0"
            as *const u8 as *const libc::c_char,
        g_defaultMaxWindowLog,
    );
    fprintf(
        stdout,
        b"  --patch-from=REF              Use REF as the reference point for Zstandard's diff engine. \n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --exclude-compressed          Only compress files that are not already compressed.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --stream-size=#               Specify size of streaming input from STDIN.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --size-hint=#                 Optimize compression parameters for streaming input of approximately size #.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --target-compressed-block-size=#\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"                                Generate compressed blocks of approximately # size.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --no-dictID                   Don't write `dictID` into the header (dictionary compression only).\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]compress-literals      Force (un)compressed literals.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]row-match-finder       Explicitly enable/disable the fast, row-based matchfinder for\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"                                the 'greedy', 'lazy', and 'lazy2' strategies.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"  --format=zstd                 Compress files to the `.zst` format. [Default]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]mmap-dict              Memory-map dictionary file rather than mallocing and loading all at once\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stdout,
        b"Advanced decompression options:\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  -l                            Print information about Zstandard-compressed files.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --test                        Test compressed file integrity.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  -M#                           Set the memory usage limit to # megabytes.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]sparse                 Enable sparse mode. [Default: Enabled for files, disabled for STDOUT.]\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut passThroughDefault = b"Disabled\0" as *const u8 as *const libc::c_char;
    if exeNameMatch(programName, ZSTD_CAT.as_ptr()) != 0
        || exeNameMatch(programName, ZSTD_ZCAT.as_ptr()) != 0
        || exeNameMatch(programName, ZSTD_GZCAT.as_ptr()) != 0
    {
        passThroughDefault = b"Enabled\0" as *const u8 as *const libc::c_char;
    }
    fprintf(
        stdout,
        b"  --[no-]pass-through           Pass through uncompressed files as-is. [Default: %s]\n\0"
            as *const u8 as *const libc::c_char,
        passThroughDefault,
    );
}
unsafe extern "C" fn badusage(mut programName: *const libc::c_char) {
    if g_displayLevel >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"Incorrect parameters \n\0" as *const u8 as *const libc::c_char,
        );
    }
    if g_displayLevel >= 2 as libc::c_int {
        usage(stderr, programName);
    }
}
unsafe extern "C" fn waitEnter() {
    let mut unused: libc::c_int = 0;
    fprintf(
        stderr,
        b"Press enter to continue... \n\0" as *const u8 as *const libc::c_char,
    );
    unused = getchar();
}
unsafe extern "C" fn lastNameFromPath(
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    let mut name = path;
    if !(strrchr(name, '/' as i32)).is_null() {
        name = (strrchr(name, '/' as i32)).offset(1 as libc::c_int as isize);
    }
    if !(strrchr(name, '\\' as i32)).is_null() {
        name = (strrchr(name, '\\' as i32)).offset(1 as libc::c_int as isize);
    }
    return name;
}
unsafe extern "C" fn errorOut(mut msg: *const libc::c_char) {
    if g_displayLevel >= 1 as libc::c_int {
        fprintf(stderr, b"%s \n\0" as *const u8 as *const libc::c_char, msg);
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn readU32FromCharChecked(
    mut stringPtr: *mut *const libc::c_char,
    mut value: *mut libc::c_uint,
) -> libc::c_int {
    let mut result = 0 as libc::c_int as libc::c_uint;
    while **stringPtr as libc::c_int >= '0' as i32
        && **stringPtr as libc::c_int <= '9' as i32
    {
        let max = (-(1 as libc::c_int) as libc::c_uint)
            .wrapping_div(10 as libc::c_int as libc::c_uint);
        let mut last = result;
        if result > max {
            return 1 as libc::c_int;
        }
        result = result.wrapping_mul(10 as libc::c_int as libc::c_uint);
        result = result
            .wrapping_add((**stringPtr as libc::c_int - '0' as i32) as libc::c_uint);
        if result < last {
            return 1 as libc::c_int;
        }
        *stringPtr = (*stringPtr).offset(1);
    }
    if **stringPtr as libc::c_int == 'K' as i32
        || **stringPtr as libc::c_int == 'M' as i32
    {
        let maxK = -(1 as libc::c_int) as libc::c_uint >> 10 as libc::c_int;
        if result > maxK {
            return 1 as libc::c_int;
        }
        result <<= 10 as libc::c_int;
        if **stringPtr as libc::c_int == 'M' as i32 {
            if result > maxK {
                return 1 as libc::c_int;
            }
            result <<= 10 as libc::c_int;
        }
        *stringPtr = (*stringPtr).offset(1);
        if **stringPtr as libc::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
        if **stringPtr as libc::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
    }
    *value = result;
    return 0 as libc::c_int;
}
unsafe extern "C" fn readU32FromChar(
    mut stringPtr: *mut *const libc::c_char,
) -> libc::c_uint {
    static mut errorMsg: [libc::c_char; 51] = unsafe {
        *::core::mem::transmute::<
            &[u8; 51],
            &[libc::c_char; 51],
        >(b"error: numeric value overflows 32-bit unsigned int\0")
    };
    let mut result: libc::c_uint = 0;
    if readU32FromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(errorMsg.as_ptr());
    }
    return result;
}
unsafe extern "C" fn readIntFromChar(
    mut stringPtr: *mut *const libc::c_char,
) -> libc::c_int {
    static mut errorMsg: [libc::c_char; 42] = unsafe {
        *::core::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"error: numeric value overflows 32-bit int\0")
    };
    let mut sign = 1 as libc::c_int;
    let mut result: libc::c_uint = 0;
    if **stringPtr as libc::c_int == '-' as i32 {
        *stringPtr = (*stringPtr).offset(1);
        sign = -(1 as libc::c_int);
    }
    if readU32FromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(errorMsg.as_ptr());
    }
    return result as libc::c_int * sign;
}
unsafe extern "C" fn readSizeTFromCharChecked(
    mut stringPtr: *mut *const libc::c_char,
    mut value: *mut size_t,
) -> libc::c_int {
    let mut result = 0 as libc::c_int as size_t;
    while **stringPtr as libc::c_int >= '0' as i32
        && **stringPtr as libc::c_int <= '9' as i32
    {
        let max = (-(1 as libc::c_int) as size_t)
            .wrapping_div(10 as libc::c_int as libc::c_ulong);
        let mut last = result;
        if result > max {
            return 1 as libc::c_int;
        }
        result = (result as libc::c_ulong)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong) as size_t as size_t;
        result = (result as libc::c_ulong)
            .wrapping_add((**stringPtr as libc::c_int - '0' as i32) as size_t) as size_t
            as size_t;
        if result < last {
            return 1 as libc::c_int;
        }
        *stringPtr = (*stringPtr).offset(1);
    }
    if **stringPtr as libc::c_int == 'K' as i32
        || **stringPtr as libc::c_int == 'M' as i32
    {
        let maxK = -(1 as libc::c_int) as size_t >> 10 as libc::c_int;
        if result > maxK {
            return 1 as libc::c_int;
        }
        result <<= 10 as libc::c_int;
        if **stringPtr as libc::c_int == 'M' as i32 {
            if result > maxK {
                return 1 as libc::c_int;
            }
            result <<= 10 as libc::c_int;
        }
        *stringPtr = (*stringPtr).offset(1);
        if **stringPtr as libc::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
        if **stringPtr as libc::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
    }
    *value = result;
    return 0 as libc::c_int;
}
unsafe extern "C" fn readSizeTFromChar(
    mut stringPtr: *mut *const libc::c_char,
) -> size_t {
    static mut errorMsg: [libc::c_char; 38] = unsafe {
        *::core::mem::transmute::<
            &[u8; 38],
            &[libc::c_char; 38],
        >(b"error: numeric value overflows size_t\0")
    };
    let mut result: size_t = 0;
    if readSizeTFromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(errorMsg.as_ptr());
    }
    return result;
}
unsafe extern "C" fn longCommandWArg(
    mut stringPtr: *mut *const libc::c_char,
    mut longCommand: *const libc::c_char,
) -> libc::c_int {
    let comSize = strlen(longCommand);
    let result = (strncmp(*stringPtr, longCommand, comSize) == 0) as libc::c_int;
    if result != 0 {
        *stringPtr = (*stringPtr).offset(comSize as isize);
    }
    return result;
}
unsafe extern "C" fn parseAdaptParameters(
    mut stringPtr: *const libc::c_char,
    mut adaptMinPtr: *mut libc::c_int,
    mut adaptMaxPtr: *mut libc::c_int,
) -> libc::c_uint {
    loop {
        if longCommandWArg(&mut stringPtr, b"min=\0" as *const u8 as *const libc::c_char)
            != 0
        {
            *adaptMinPtr = readIntFromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"max=\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            *adaptMaxPtr = readIntFromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            if g_displayLevel >= 4 as libc::c_int {
                fprintf(
                    stderr,
                    b"invalid compression parameter \n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    if *stringPtr.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    if *adaptMinPtr > *adaptMaxPtr {
        if g_displayLevel >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"incoherent adaptation limits \n\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn parseCompressionParameters(
    mut stringPtr: *const libc::c_char,
    mut params: *mut ZSTD_compressionParameters,
) -> libc::c_uint {
    loop {
        if longCommandWArg(
            &mut stringPtr,
            b"windowLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"wlog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).windowLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"chainLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"clog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).chainLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"hashLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"hlog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).hashLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"searchLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"slog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).searchLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"minMatch=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"mml=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).minMatch = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"targetLength=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"tlen=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).targetLength = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"strategy=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"strat=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            (*params).strategy = readU32FromChar(&mut stringPtr) as ZSTD_strategy;
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"overlapLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"ovlog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            g_overlapLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmHashLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lhlog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            g_ldmHashLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmMinMatch=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lmml=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            g_ldmMinMatch = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmBucketSizeLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lblog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            g_ldmBucketSizeLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmHashRateLog=\0" as *const u8 as *const libc::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lhrlog=\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            g_ldmHashRateLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32)
            {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            if g_displayLevel >= 4 as libc::c_int {
                fprintf(
                    stderr,
                    b"invalid compression parameter \n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    if g_displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"windowLog=%d, chainLog=%d, hashLog=%d, searchLog=%d \n\0" as *const u8
                as *const libc::c_char,
            (*params).windowLog,
            (*params).chainLog,
            (*params).hashLog,
            (*params).searchLog,
        );
    }
    if g_displayLevel >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"minMatch=%d, targetLength=%d, strategy=%d \n\0" as *const u8
                as *const libc::c_char,
            (*params).minMatch,
            (*params).targetLength,
            (*params).strategy as libc::c_uint,
        );
    }
    if *stringPtr.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn printVersion() {
    if g_displayLevel < DISPLAY_LEVEL_DEFAULT {
        fprintf(
            stdout,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b"1.5.5\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fprintf(
        stdout,
        b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8 as *const libc::c_char,
        b"Zstandard CLI\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"v1.5.5\0" as *const u8 as *const libc::c_char,
        b"Yann Collet\0" as *const u8 as *const libc::c_char,
    );
    if g_displayLevel >= 3 as libc::c_int {
        fprintf(stdout, b"*** supports: zstd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b", zstd legacy v0.%d+\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        if g_displayLevel >= 4 as libc::c_int {
            fprintf(
                stdout,
                b"zlib version %s\n\0" as *const u8 as *const libc::c_char,
                FIO_zlibVersion(),
            );
            fprintf(
                stdout,
                b"lz4 version %s\n\0" as *const u8 as *const libc::c_char,
                FIO_lz4Version(),
            );
            fprintf(
                stdout,
                b"lzma version %s\n\0" as *const u8 as *const libc::c_char,
                FIO_lzmaVersion(),
            );
            fprintf(
                stdout,
                b"_POSIX_C_SOURCE defined: %ldL\n\0" as *const u8 as *const libc::c_char,
                200809 as libc::c_long,
            );
            fprintf(
                stdout,
                b"_POSIX_VERSION defined: %ldL \n\0" as *const u8 as *const libc::c_char,
                200809 as libc::c_long,
            );
            fprintf(
                stdout,
                b"PLATFORM_POSIX_VERSION defined: %ldL\n\0" as *const u8
                    as *const libc::c_char,
                200809 as libc::c_long,
            );
        }
    }
}
static mut ZSTD_strategyMap: [*const libc::c_char; 10] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"ZSTD_fast\0" as *const u8 as *const libc::c_char,
    b"ZSTD_dfast\0" as *const u8 as *const libc::c_char,
    b"ZSTD_greedy\0" as *const u8 as *const libc::c_char,
    b"ZSTD_lazy\0" as *const u8 as *const libc::c_char,
    b"ZSTD_lazy2\0" as *const u8 as *const libc::c_char,
    b"ZSTD_btlazy2\0" as *const u8 as *const libc::c_char,
    b"ZSTD_btopt\0" as *const u8 as *const libc::c_char,
    b"ZSTD_btultra\0" as *const u8 as *const libc::c_char,
    b"ZSTD_btultra2\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn printDefaultCParams(
    mut filename: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
    mut cLevel: libc::c_int,
) {
    let mut fileSize = UTIL_getFileSize(filename) as libc::c_ulonglong;
    let dictSize = if !dictFileName.is_null() {
        UTIL_getFileSize(dictFileName)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let cParams = ZSTD_getCParams(cLevel, fileSize, dictSize);
    if fileSize != UTIL_FILESIZE_UNKNOWN as libc::c_ulonglong {
        fprintf(
            stderr,
            b"%s (%u bytes)\n\0" as *const u8 as *const libc::c_char,
            filename,
            fileSize as libc::c_uint,
        );
    } else {
        fprintf(
            stderr,
            b"%s (src size unknown)\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
    }
    fprintf(
        stderr,
        b" - windowLog     : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.windowLog,
    );
    fprintf(
        stderr,
        b" - chainLog      : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.chainLog,
    );
    fprintf(
        stderr,
        b" - hashLog       : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.hashLog,
    );
    fprintf(
        stderr,
        b" - searchLog     : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.searchLog,
    );
    fprintf(
        stderr,
        b" - minMatch      : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.minMatch,
    );
    fprintf(
        stderr,
        b" - targetLength  : %u\n\0" as *const u8 as *const libc::c_char,
        cParams.targetLength,
    );
    if (cParams.strategy as libc::c_uint)
        < (9 as libc::c_int + 1 as libc::c_int) as libc::c_uint
    {} else {
        __assert_fail(
            b"cParams.strategy < ZSTD_NB_STRATEGIES + 1\0" as *const u8
                as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0" as *const u8
                as *const libc::c_char,
            707 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void printDefaultCParams(const char *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    fprintf(
        stderr,
        b" - strategy      : %s (%u)\n\0" as *const u8 as *const libc::c_char,
        ZSTD_strategyMap[cParams.strategy as libc::c_int as usize],
        cParams.strategy as libc::c_uint,
    );
}
unsafe extern "C" fn printActualCParams(
    mut filename: *const libc::c_char,
    mut dictFileName: *const libc::c_char,
    mut cLevel: libc::c_int,
    mut cParams: *const ZSTD_compressionParameters,
) {
    let mut fileSize = UTIL_getFileSize(filename) as libc::c_ulonglong;
    let dictSize = if !dictFileName.is_null() {
        UTIL_getFileSize(dictFileName)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut actualCParams = ZSTD_getCParams(cLevel, fileSize, dictSize);
    if g_displayLevel >= 4 as libc::c_int {} else {
        __assert_fail(
            b"g_displayLevel >= 4\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0" as *const u8
                as *const libc::c_char,
            715 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"void printActualCParams(const char *, const char *, int, const ZSTD_compressionParameters *)\0",
            ))
                .as_ptr(),
        );
    }
    actualCParams
        .windowLog = if (*cParams).windowLog == 0 as libc::c_int as libc::c_uint {
        actualCParams.windowLog
    } else {
        (*cParams).windowLog
    };
    actualCParams
        .chainLog = if (*cParams).chainLog == 0 as libc::c_int as libc::c_uint {
        actualCParams.chainLog
    } else {
        (*cParams).chainLog
    };
    actualCParams
        .hashLog = if (*cParams).hashLog == 0 as libc::c_int as libc::c_uint {
        actualCParams.hashLog
    } else {
        (*cParams).hashLog
    };
    actualCParams
        .searchLog = if (*cParams).searchLog == 0 as libc::c_int as libc::c_uint {
        actualCParams.searchLog
    } else {
        (*cParams).searchLog
    };
    actualCParams
        .minMatch = if (*cParams).minMatch == 0 as libc::c_int as libc::c_uint {
        actualCParams.minMatch
    } else {
        (*cParams).minMatch
    };
    actualCParams
        .targetLength = if (*cParams).targetLength == 0 as libc::c_int as libc::c_uint {
        actualCParams.targetLength
    } else {
        (*cParams).targetLength
    };
    actualCParams
        .strategy = (if (*cParams).strategy as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        actualCParams.strategy as libc::c_uint
    } else {
        (*cParams).strategy as libc::c_uint
    }) as ZSTD_strategy;
    fprintf(
        stderr,
        b"--zstd=wlog=%d,clog=%d,hlog=%d,slog=%d,mml=%d,tlen=%d,strat=%d\n\0"
            as *const u8 as *const libc::c_char,
        actualCParams.windowLog,
        actualCParams.chainLog,
        actualCParams.hashLog,
        actualCParams.searchLog,
        actualCParams.minMatch,
        actualCParams.targetLength,
        actualCParams.strategy as libc::c_uint,
    );
}
pub const ENV_CLEVEL: [libc::c_char; 12] = unsafe {
    *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ZSTD_CLEVEL\0")
};
unsafe extern "C" fn init_cLevel() -> libc::c_int {
    let env: *const libc::c_char = getenv(ENV_CLEVEL.as_ptr());
    if !env.is_null() {
        let mut ptr = env;
        let mut sign = 1 as libc::c_int;
        if *ptr as libc::c_int == '-' as i32 {
            sign = -(1 as libc::c_int);
            ptr = ptr.offset(1);
        } else if *ptr as libc::c_int == '+' as i32 {
            ptr = ptr.offset(1);
        }
        if *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32 {
            let mut absLevel: libc::c_uint = 0;
            if readU32FromCharChecked(&mut ptr, &mut absLevel) != 0 {
                if g_displayLevel >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Ignore environment variable setting %s=%s: numeric value too large \n\0"
                            as *const u8 as *const libc::c_char,
                        b"ZSTD_CLEVEL\0" as *const u8 as *const libc::c_char,
                        env,
                    );
                }
                return ZSTDCLI_CLEVEL_DEFAULT;
            } else {
                if *ptr as libc::c_int == 0 as libc::c_int {
                    return sign * absLevel as libc::c_int;
                }
            }
        }
        if g_displayLevel >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"Ignore environment variable setting %s=%s: not a valid integer value \n\0"
                    as *const u8 as *const libc::c_char,
                b"ZSTD_CLEVEL\0" as *const u8 as *const libc::c_char,
                env,
            );
        }
    }
    return ZSTDCLI_CLEVEL_DEFAULT;
}
pub const MINCLEVEL: libc::c_int = ZSTD_minCLevel();
pub const MAXCLEVEL: libc::c_int = ZSTD_maxCLevel();
unsafe fn main_0(
    mut argCount: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut argNb: libc::c_int = 0;
    let mut followLinks = 0 as libc::c_int;
    let mut allowBlockDevices = 0 as libc::c_int;
    let mut forceStdin = 0 as libc::c_int;
    let mut forceStdout = 0 as libc::c_int;
    let mut hasStdout = 0 as libc::c_int;
    let mut ldmFlag = 0 as libc::c_int;
    let mut main_pause = 0 as libc::c_int;
    let mut adapt = 0 as libc::c_int;
    let mut adaptMin = MINCLEVEL;
    let mut adaptMax = MAXCLEVEL;
    let mut rsyncable = 0 as libc::c_int;
    let mut nextArgumentsAreFiles = 0 as libc::c_int;
    let mut operationResult = 0 as libc::c_int;
    let mut separateFiles = 0 as libc::c_int;
    let mut setRealTimePrio = 0 as libc::c_int;
    let mut singleThread = 0 as libc::c_int;
    let mut defaultLogicalCores = 0 as libc::c_int;
    let mut showDefaultCParams = 0 as libc::c_int;
    let mut ultra = 0 as libc::c_int;
    let mut contentSize = 1 as libc::c_int;
    let mut removeSrcFile = 0 as libc::c_int;
    let mut mmapDict = ZSTD_ps_auto;
    let mut useRowMatchFinder = ZSTD_ps_auto;
    let mut cType = FIO_zstdCompression;
    let mut nbWorkers = 0 as libc::c_int as libc::c_uint;
    let mut compressibility = 0.5f64;
    let mut bench_nbSeconds = 3 as libc::c_int as libc::c_uint;
    let mut blockSize = 0 as libc::c_int as size_t;
    let prefs = FIO_createPreferences();
    let fCtx = FIO_createContext();
    let mut progress = FIO_ps_auto;
    let mut operation = zom_compress;
    let mut compressionParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0 as ZSTD_strategy,
    };
    let mut cLevel = init_cLevel();
    let mut cLevelLast = MINCLEVEL - 1 as libc::c_int;
    let mut recursive = 0 as libc::c_int as libc::c_uint;
    let mut memLimit = 0 as libc::c_int as libc::c_uint;
    let mut filenames = UTIL_allocateFileNamesTable(argCount as size_t);
    let mut file_of_names = UTIL_allocateFileNamesTable(argCount as size_t);
    let mut programName = *argv.offset(0 as libc::c_int as isize);
    let mut outFileName = NULL as *const libc::c_char;
    let mut outDirName = NULL as *const libc::c_char;
    let mut outMirroredDirName = NULL as *const libc::c_char;
    let mut dictFileName = NULL as *const libc::c_char;
    let mut patchFromDictFileName = NULL as *const libc::c_char;
    let mut suffix = ZSTD_EXTENSION.as_ptr();
    let mut maxDictSize = g_defaultMaxDictSize;
    let mut dictID = 0 as libc::c_int as libc::c_uint;
    let mut streamSrcSize = 0 as libc::c_int as size_t;
    let mut targetCBlockSize = 0 as libc::c_int as size_t;
    let mut srcSizeHint = 0 as libc::c_int as size_t;
    let mut nbInputFileNames = 0 as libc::c_int as size_t;
    let mut dictCLevel = g_defaultDictCLevel;
    let mut dictSelect = g_defaultSelectivityLevel;
    let mut literalCompressionMode = ZSTD_ps_auto;
    checkLibVersion();
    if argCount >= 1 as libc::c_int {} else {
        __assert_fail(
            b"argCount >= 1\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0" as *const u8
                as *const libc::c_char,
            904 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"int main(int, const char **)\0"))
                .as_ptr(),
        );
    }
    if filenames.is_null() || file_of_names.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"zstd: allocation error \n\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    programName = lastNameFromPath(programName);
    if exeNameMatch(programName, ZSTD_ZSTDMT.as_ptr()) != 0 {
        nbWorkers = 0 as libc::c_int as libc::c_uint;
        singleThread = 0 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_UNZSTD.as_ptr()) != 0 {
        operation = zom_decompress;
    }
    if exeNameMatch(programName, ZSTD_CAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1 as libc::c_int;
        followLinks = 1 as libc::c_int;
        FIO_setPassThroughFlag(prefs, 1 as libc::c_int);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_ZCAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1 as libc::c_int;
        followLinks = 1 as libc::c_int;
        FIO_setPassThroughFlag(prefs, 1 as libc::c_int);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_GZ.as_ptr()) != 0 {
        suffix = GZ_EXTENSION.as_ptr();
        cType = FIO_gzipCompression;
        removeSrcFile = 1 as libc::c_int;
        cLevel = 6 as libc::c_int;
        dictCLevel = cLevel;
    }
    if exeNameMatch(programName, ZSTD_GUNZIP.as_ptr()) != 0 {
        operation = zom_decompress;
        removeSrcFile = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_GZCAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1 as libc::c_int;
        followLinks = 1 as libc::c_int;
        FIO_setPassThroughFlag(prefs, 1 as libc::c_int);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_LZMA.as_ptr()) != 0 {
        suffix = LZMA_EXTENSION.as_ptr();
        cType = FIO_lzmaCompression;
        removeSrcFile = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_UNLZMA.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_lzmaCompression;
        removeSrcFile = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_XZ.as_ptr()) != 0 {
        suffix = XZ_EXTENSION.as_ptr();
        cType = FIO_xzCompression;
        removeSrcFile = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_UNXZ.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_xzCompression;
        removeSrcFile = 1 as libc::c_int;
    }
    if exeNameMatch(programName, ZSTD_LZ4.as_ptr()) != 0 {
        suffix = LZ4_EXTENSION.as_ptr();
        cType = FIO_lz4Compression;
    }
    if exeNameMatch(programName, ZSTD_UNLZ4.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_lz4Compression;
    }
    memset(
        &mut compressionParams as *mut ZSTD_compressionParameters as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ZSTD_compressionParameters>() as libc::c_ulong,
    );
    FIO_addAbortHandler();
    argNb = 1 as libc::c_int;
    's_369: loop {
        if !(argNb < argCount) {
            current_block = 2356153619472235877;
            break;
        }
        let mut argument = *argv.offset(argNb as isize);
        if !argument.is_null() {
            if nextArgumentsAreFiles != 0 {
                UTIL_refFilename(filenames, argument);
            } else if strcmp(argument, b"-\0" as *const u8 as *const libc::c_char) == 0 {
                UTIL_refFilename(filenames, stdinmark.as_ptr());
            } else if *argument.offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
            {
                if *argument.offset(1 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                {
                    if strcmp(argument, b"--\0" as *const u8 as *const libc::c_char) == 0
                    {
                        nextArgumentsAreFiles = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--list\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_list;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--compress\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_compress;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--decompress\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_decompress;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--uncompress\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_decompress;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--force\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_overwriteMode(prefs);
                        forceStdin = 1 as libc::c_int;
                        forceStdout = 1 as libc::c_int;
                        followLinks = 1 as libc::c_int;
                        allowBlockDevices = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--version\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        printVersion();
                        operationResult = 0 as libc::c_int;
                        current_block = 18342783468770781838;
                        break;
                    } else if strcmp(
                        argument,
                        b"--help\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        usage_advanced(programName);
                        operationResult = 0 as libc::c_int;
                        current_block = 18342783468770781838;
                        break;
                    } else if strcmp(
                        argument,
                        b"--verbose\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        g_displayLevel += 1;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--quiet\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        g_displayLevel -= 1;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--stdout\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        forceStdout = 1 as libc::c_int;
                        outFileName = stdoutmark.as_ptr();
                        removeSrcFile = 0 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--ultra\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        ultra = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--check\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setChecksumFlag(prefs, 2 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-check\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setChecksumFlag(prefs, 0 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--sparse\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setSparseWrite(prefs, 2 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-sparse\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setSparseWrite(prefs, 0 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--pass-through\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setPassThroughFlag(prefs, 1 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-pass-through\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setPassThroughFlag(prefs, 0 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--test\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_test;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--asyncio\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setAsyncIOFlag(prefs, 1 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-asyncio\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setAsyncIOFlag(prefs, 0 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--train\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        operation = zom_train;
                        if outFileName.is_null() {
                            outFileName = g_defaultDictName;
                        }
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-dictID\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setDictIDFlag(prefs, 0 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--keep\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        removeSrcFile = 0 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--rm\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        removeSrcFile = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--priority=rt\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        setRealTimePrio = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--show-default-cparams\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        showDefaultCParams = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--content-size\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        contentSize = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-content-size\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        contentSize = 0 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--adapt\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        adapt = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-row-match-finder\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        useRowMatchFinder = ZSTD_ps_disable;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--row-match-finder\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        useRowMatchFinder = ZSTD_ps_enable;
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--adapt=\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        adapt = 1 as libc::c_int;
                        if parseAdaptParameters(argument, &mut adaptMin, &mut adaptMax)
                            == 0
                        {
                            badusage(programName);
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            current_block = 3229571381435211107;
                        }
                    } else if strcmp(
                        argument,
                        b"--single-thread\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        nbWorkers = 0 as libc::c_int as libc::c_uint;
                        singleThread = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--format=zstd\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        suffix = ZSTD_EXTENSION.as_ptr();
                        cType = FIO_zstdCompression;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--mmap-dict\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        mmapDict = ZSTD_ps_enable;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-mmap-dict\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        mmapDict = ZSTD_ps_disable;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--rsyncable\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        rsyncable = 1 as libc::c_int;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--compress-literals\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        literalCompressionMode = ZSTD_ps_enable;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-compress-literals\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        literalCompressionMode = ZSTD_ps_disable;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--no-progress\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        progress = FIO_ps_never;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--progress\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        progress = FIO_ps_always;
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--exclude-compressed\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        FIO_setExcludeCompressedFile(prefs, 1 as libc::c_int);
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--fake-stdin-is-console\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        UTIL_fakeStdinIsConsole();
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--fake-stdout-is-console\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        UTIL_fakeStdoutIsConsole();
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--fake-stderr-is-console\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        UTIL_fakeStderrIsConsole();
                        current_block = 3229571381435211107;
                    } else if strcmp(
                        argument,
                        b"--trace-file-stat\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        UTIL_traceFileStat();
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--threads\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb = argument;
                            argument = argument.offset(strlen(__nb) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb = *argv.offset(argNb as isize);
                                if !__nb.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1052 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        nbWorkers = readU32FromChar(&mut __nb);
                        if *__nb as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--memlimit\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_0 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_0 = argument;
                            argument = argument.offset(strlen(__nb_0) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_0 = *argv.offset(argNb as isize);
                                if !__nb_0.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1053 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_0.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        memLimit = readU32FromChar(&mut __nb_0);
                        if *__nb_0 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--memory\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_1 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_1 = argument;
                            argument = argument.offset(strlen(__nb_1) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_1 = *argv.offset(argNb as isize);
                                if !__nb_1.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1054 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_1.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        memLimit = readU32FromChar(&mut __nb_1);
                        if *__nb_1 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--memlimit-decompress\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_2 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_2 = argument;
                            argument = argument.offset(strlen(__nb_2) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_2 = *argv.offset(argNb as isize);
                                if !__nb_2.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1055 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_2.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        memLimit = readU32FromChar(&mut __nb_2);
                        if *__nb_2 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--block-size\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_3 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_3 = argument;
                            argument = argument.offset(strlen(__nb_3) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_3 = *argv.offset(argNb as isize);
                                if !__nb_3.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1056 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_3.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        blockSize = readSizeTFromChar(&mut __nb_3);
                        if *__nb_3 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--maxdict\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_4 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_4 = argument;
                            argument = argument.offset(strlen(__nb_4) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_4 = *argv.offset(argNb as isize);
                                if !__nb_4.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1057 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_4.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        maxDictSize = readU32FromChar(&mut __nb_4);
                        if *__nb_4 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--dictID\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_5 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_5 = argument;
                            argument = argument.offset(strlen(__nb_5) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_5 = *argv.offset(argNb as isize);
                                if !__nb_5.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1058 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_5.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        dictID = readU32FromChar(&mut __nb_5);
                        if *__nb_5 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--zstd=\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        if parseCompressionParameters(argument, &mut compressionParams)
                            == 0
                        {
                            badusage(programName);
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            cType = FIO_zstdCompression;
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--stream-size\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_6 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_6 = argument;
                            argument = argument.offset(strlen(__nb_6) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_6 = *argv.offset(argNb as isize);
                                if !__nb_6.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1060 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_6.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        streamSrcSize = readSizeTFromChar(&mut __nb_6);
                        if *__nb_6 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--target-compressed-block-size\0" as *const u8
                            as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_7 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_7 = argument;
                            argument = argument.offset(strlen(__nb_7) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_7 = *argv.offset(argNb as isize);
                                if !__nb_7.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1061 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_7.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        targetCBlockSize = readSizeTFromChar(&mut __nb_7);
                        if *__nb_7 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--size-hint\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut __nb_8 = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            __nb_8 = argument;
                            argument = argument.offset(strlen(__nb_8) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                __nb_8 = *argv.offset(argNb as isize);
                                if !__nb_8.is_null() {} else {
                                    __assert_fail(
                                        b"__nb != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1062 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *__nb_8.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        srcSizeHint = readSizeTFromChar(&mut __nb_8);
                        if *__nb_8 as libc::c_int != 0 as libc::c_int {
                            errorOut(
                                b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--output-dir-flat\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            outDirName = argument;
                            argument = argument.offset(strlen(outDirName) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                outDirName = *argv.offset(argNb as isize);
                                if !outDirName.is_null() {} else {
                                    __assert_fail(
                                        b"outDirName != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1064 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *outDirName.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        if strlen(outDirName) == 0 as libc::c_int as libc::c_ulong {
                            if g_displayLevel >= 1 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"error: output dir cannot be empty string (did you mean to pass '.' instead?)\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            current_block = 3229571381435211107;
                        }
                    } else if longCommandWArg(
                        &mut argument,
                        b"--auto-threads\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut threadDefault = NULL as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            threadDefault = argument;
                            argument = argument.offset(strlen(threadDefault) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                threadDefault = *argv.offset(argNb as isize);
                                if !threadDefault.is_null() {} else {
                                    __assert_fail(
                                        b"threadDefault != NULL\0" as *const u8
                                            as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1073 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *threadDefault.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        if strcmp(
                            threadDefault,
                            b"logical\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            defaultLogicalCores = 1 as libc::c_int;
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--output-dir-mirror\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            outMirroredDirName = argument;
                            argument = argument
                                .offset(strlen(outMirroredDirName) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                outMirroredDirName = *argv.offset(argNb as isize);
                                if !outMirroredDirName.is_null() {} else {
                                    __assert_fail(
                                        b"outMirroredDirName != NULL\0" as *const u8
                                            as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1080 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *outMirroredDirName.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        if strlen(outMirroredDirName)
                            == 0 as libc::c_int as libc::c_ulong
                        {
                            if g_displayLevel >= 1 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"error: output dir cannot be empty string (did you mean to pass '.' instead?)\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            current_block = 3229571381435211107;
                        }
                    } else if longCommandWArg(
                        &mut argument,
                        b"--patch-from\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            patchFromDictFileName = argument;
                            argument = argument
                                .offset(strlen(patchFromDictFileName) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                patchFromDictFileName = *argv.offset(argNb as isize);
                                if !patchFromDictFileName.is_null() {} else {
                                    __assert_fail(
                                        b"patchFromDictFileName != NULL\0" as *const u8
                                            as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1091 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *patchFromDictFileName.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--long\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut ldmWindowLog = 0 as libc::c_int as libc::c_uint;
                        ldmFlag = 1 as libc::c_int;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            ldmWindowLog = readU32FromChar(&mut argument);
                        } else if *argument as libc::c_int != 0 as libc::c_int {
                            badusage(programName);
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            ldmWindowLog = g_defaultMaxWindowLog;
                        }
                        if compressionParams.windowLog
                            == 0 as libc::c_int as libc::c_uint
                        {
                            compressionParams.windowLog = ldmWindowLog;
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--fast\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        if *argument as libc::c_int == '=' as i32 {
                            let maxFast = -ZSTD_minCLevel() as U32;
                            let mut fastLevel: U32 = 0;
                            argument = argument.offset(1);
                            fastLevel = readU32FromChar(&mut argument);
                            if fastLevel > maxFast {
                                fastLevel = maxFast;
                            }
                            if fastLevel != 0 {
                                cLevel = -(fastLevel as libc::c_int);
                                dictCLevel = cLevel;
                            } else {
                                badusage(programName);
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            }
                        } else if *argument as libc::c_int != 0 as libc::c_int {
                            badusage(programName);
                            operationResult = 1 as libc::c_int;
                            current_block = 18342783468770781838;
                            break;
                        } else {
                            cLevel = -(1 as libc::c_int);
                        }
                        current_block = 3229571381435211107;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--filelist\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        let mut listName = 0 as *const libc::c_char;
                        if *argument as libc::c_int == '=' as i32 {
                            argument = argument.offset(1);
                            listName = argument;
                            argument = argument.offset(strlen(listName) as isize);
                        } else {
                            argNb += 1;
                            if argNb >= argCount {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"error: missing command argument \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                listName = *argv.offset(argNb as isize);
                                if !listName.is_null() {} else {
                                    __assert_fail(
                                        b"listName != NULL\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1139 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"int main(int, const char **)\0"))
                                            .as_ptr(),
                                    );
                                }
                                if *listName.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"error: command cannot be separated from its argument by another command \n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                    current_block = 18342783468770781838;
                                    break;
                                }
                            }
                        }
                        UTIL_refFilename(file_of_names, listName);
                        current_block = 3229571381435211107;
                    } else {
                        current_block = 12366097880291256458;
                    }
                } else {
                    current_block = 12366097880291256458;
                }
                match current_block {
                    3229571381435211107 => {}
                    _ => {
                        argument = argument.offset(1);
                        while *argument.offset(0 as libc::c_int as isize) as libc::c_int
                            != 0 as libc::c_int
                        {
                            if *argument as libc::c_int >= '0' as i32
                                && *argument as libc::c_int <= '9' as i32
                            {
                                cLevel = readU32FromChar(&mut argument) as libc::c_int;
                                dictCLevel = cLevel;
                            } else {
                                match *argument.offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                {
                                    86 => {
                                        printVersion();
                                        operationResult = 0 as libc::c_int;
                                        current_block = 18342783468770781838;
                                        break 's_369;
                                    }
                                    72 => {
                                        usage_advanced(programName);
                                        operationResult = 0 as libc::c_int;
                                        current_block = 18342783468770781838;
                                        break 's_369;
                                    }
                                    104 => {
                                        usage(stdout, programName);
                                        operationResult = 0 as libc::c_int;
                                        current_block = 18342783468770781838;
                                        break 's_369;
                                    }
                                    122 => {
                                        operation = zom_compress;
                                        argument = argument.offset(1);
                                    }
                                    100 => {
                                        operation = zom_decompress;
                                        argument = argument.offset(1);
                                    }
                                    99 => {
                                        forceStdout = 1 as libc::c_int;
                                        outFileName = stdoutmark.as_ptr();
                                        removeSrcFile = 0 as libc::c_int;
                                        argument = argument.offset(1);
                                    }
                                    110 => {
                                        argument = argument.offset(1);
                                    }
                                    68 => {
                                        argument = argument.offset(1);
                                        if *argument as libc::c_int == '=' as i32 {
                                            argument = argument.offset(1);
                                            dictFileName = argument;
                                            argument = argument.offset(strlen(dictFileName) as isize);
                                        } else {
                                            argNb += 1;
                                            if argNb >= argCount {
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"error: missing command argument \n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                                current_block = 18342783468770781838;
                                                break 's_369;
                                            } else {
                                                dictFileName = *argv.offset(argNb as isize);
                                                if !dictFileName.is_null() {} else {
                                                    __assert_fail(
                                                        b"dictFileName != NULL\0" as *const u8
                                                            as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1183 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 29],
                                                            &[libc::c_char; 29],
                                                        >(b"int main(int, const char **)\0"))
                                                            .as_ptr(),
                                                    );
                                                }
                                                if !(*dictFileName.offset(0 as libc::c_int as isize)
                                                    as libc::c_int == '-' as i32)
                                                {
                                                    continue;
                                                }
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                                current_block = 18342783468770781838;
                                                break 's_369;
                                            }
                                        }
                                    }
                                    102 => {
                                        FIO_overwriteMode(prefs);
                                        forceStdin = 1 as libc::c_int;
                                        forceStdout = 1 as libc::c_int;
                                        followLinks = 1 as libc::c_int;
                                        allowBlockDevices = 1 as libc::c_int;
                                        argument = argument.offset(1);
                                    }
                                    118 => {
                                        g_displayLevel += 1;
                                        argument = argument.offset(1);
                                    }
                                    113 => {
                                        g_displayLevel -= 1;
                                        argument = argument.offset(1);
                                    }
                                    107 => {
                                        removeSrcFile = 0 as libc::c_int;
                                        argument = argument.offset(1);
                                    }
                                    67 => {
                                        FIO_setChecksumFlag(prefs, 2 as libc::c_int);
                                        argument = argument.offset(1);
                                    }
                                    116 => {
                                        operation = zom_test;
                                        argument = argument.offset(1);
                                    }
                                    111 => {
                                        argument = argument.offset(1);
                                        if *argument as libc::c_int == '=' as i32 {
                                            argument = argument.offset(1);
                                            outFileName = argument;
                                            argument = argument.offset(strlen(outFileName) as isize);
                                        } else {
                                            argNb += 1;
                                            if argNb >= argCount {
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"error: missing command argument \n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                                current_block = 18342783468770781838;
                                                break 's_369;
                                            } else {
                                                outFileName = *argv.offset(argNb as isize);
                                                if !outFileName.is_null() {} else {
                                                    __assert_fail(
                                                        b"outFileName != NULL\0" as *const u8
                                                            as *const libc::c_char,
                                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1204 as libc::c_int as libc::c_uint,
                                                        (*::core::mem::transmute::<
                                                            &[u8; 29],
                                                            &[libc::c_char; 29],
                                                        >(b"int main(int, const char **)\0"))
                                                            .as_ptr(),
                                                    );
                                                }
                                                if !(*outFileName.offset(0 as libc::c_int as isize)
                                                    as libc::c_int == '-' as i32)
                                                {
                                                    continue;
                                                }
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                                current_block = 18342783468770781838;
                                                break 's_369;
                                            }
                                        }
                                    }
                                    77 => {
                                        argument = argument.offset(1);
                                        memLimit = readU32FromChar(&mut argument);
                                    }
                                    108 => {
                                        operation = zom_list;
                                        argument = argument.offset(1);
                                    }
                                    114 => {
                                        recursive = 1 as libc::c_int as libc::c_uint;
                                        argument = argument.offset(1);
                                    }
                                    84 => {
                                        argument = argument.offset(1);
                                        nbWorkers = readU32FromChar(&mut argument);
                                    }
                                    115 => {
                                        argument = argument.offset(1);
                                        dictSelect = readU32FromChar(&mut argument);
                                    }
                                    112 => {
                                        argument = argument.offset(1);
                                        main_pause = 1 as libc::c_int;
                                    }
                                    80 => {
                                        argument = argument.offset(1);
                                        compressibility = readU32FromChar(&mut argument)
                                            as libc::c_double / 100 as libc::c_int as libc::c_double;
                                    }
                                    _ => {
                                        badusage(programName);
                                        operationResult = 1 as libc::c_int;
                                        current_block = 18342783468770781838;
                                        break 's_369;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                UTIL_refFilename(filenames, argument);
            }
        }
        argNb += 1;
    }
    match current_block {
        2356153619472235877 => {
            if g_displayLevel >= 3 as libc::c_int {
                fprintf(
                    stderr,
                    b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8
                        as *const libc::c_char,
                    b"Zstandard CLI\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int,
                    b"v1.5.5\0" as *const u8 as *const libc::c_char,
                    b"Yann Collet\0" as *const u8 as *const libc::c_char,
                );
            }
            g_utilDisplayLevel = g_displayLevel;
            if followLinks == 0 {
                let mut u: libc::c_uint = 0;
                let mut fileNamesNb: libc::c_uint = 0;
                let nbFilenames = (*filenames).tableSize as libc::c_uint;
                u = 0 as libc::c_int as libc::c_uint;
                fileNamesNb = 0 as libc::c_int as libc::c_uint;
                while u < nbFilenames {
                    if UTIL_isLink(*((*filenames).fileNames).offset(u as isize)) != 0
                        && UTIL_isFIFO(*((*filenames).fileNames).offset(u as isize)) == 0
                    {
                        if g_displayLevel >= 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Warning : %s is a symbolic link, ignoring \n\0"
                                    as *const u8 as *const libc::c_char,
                                *((*filenames).fileNames).offset(u as isize),
                            );
                        }
                    } else {
                        let fresh0 = fileNamesNb;
                        fileNamesNb = fileNamesNb.wrapping_add(1);
                        let ref mut fresh1 = *((*filenames).fileNames)
                            .offset(fresh0 as isize);
                        *fresh1 = *((*filenames).fileNames).offset(u as isize);
                    }
                    u = u.wrapping_add(1);
                }
                if fileNamesNb == 0 as libc::c_int as libc::c_uint
                    && nbFilenames > 0 as libc::c_int as libc::c_uint
                {
                    operationResult = 1 as libc::c_int;
                    current_block = 18342783468770781838;
                } else {
                    (*filenames).tableSize = fileNamesNb as size_t;
                    current_block = 4526666330231775022;
                }
            } else {
                current_block = 4526666330231775022;
            }
            match current_block {
                18342783468770781838 => {}
                _ => {
                    if (*file_of_names).tableSize != 0 {
                        let nbFileLists = (*file_of_names).tableSize;
                        let mut flNb: size_t = 0;
                        flNb = 0 as libc::c_int as size_t;
                        loop {
                            if !(flNb < nbFileLists) {
                                current_block = 3266459246600429215;
                                break;
                            }
                            let fnt = UTIL_createFileNamesTable_fromFileName(
                                *((*file_of_names).fileNames).offset(flNb as isize),
                            );
                            if fnt.is_null() {
                                if g_displayLevel >= 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"zstd: error reading %s \n\0" as *const u8
                                            as *const libc::c_char,
                                        *((*file_of_names).fileNames).offset(flNb as isize),
                                    );
                                }
                                operationResult = 1 as libc::c_int;
                                current_block = 18342783468770781838;
                                break;
                            } else {
                                filenames = UTIL_mergeFileNamesTable(filenames, fnt);
                                flNb = flNb.wrapping_add(1);
                            }
                        }
                    } else {
                        current_block = 3266459246600429215;
                    }
                    match current_block {
                        18342783468770781838 => {}
                        _ => {
                            nbInputFileNames = (*filenames).tableSize;
                            if recursive != 0 {
                                UTIL_expandFNT(&mut filenames, followLinks);
                            }
                            if operation as libc::c_uint
                                == zom_list as libc::c_int as libc::c_uint
                            {
                                let ret = FIO_listMultipleFiles(
                                    (*filenames).tableSize as libc::c_uint,
                                    (*filenames).fileNames,
                                    g_displayLevel,
                                );
                                operationResult = ret;
                            } else if !(operation as libc::c_uint
                                == zom_bench as libc::c_int as libc::c_uint)
                            {
                                if operation as libc::c_uint
                                    == zom_train as libc::c_int as libc::c_uint
                                {
                                    if g_displayLevel >= 1 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            b"training mode not available \n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    operationResult = 1 as libc::c_int;
                                } else {
                                    if operation as libc::c_uint
                                        == zom_test as libc::c_int as libc::c_uint
                                    {
                                        FIO_setTestMode(prefs, 1 as libc::c_int);
                                        outFileName = nulmark.as_ptr();
                                        removeSrcFile = 0 as libc::c_int;
                                    }
                                    if (*filenames).tableSize
                                        == 0 as libc::c_int as libc::c_ulong
                                    {
                                        if nbInputFileNames > 0 as libc::c_int as libc::c_ulong {
                                            if g_displayLevel >= 1 as libc::c_int {
                                                fprintf(
                                                    stderr,
                                                    b"please provide correct input file(s) or non-empty directories -- ignored \n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            operationResult = 0 as libc::c_int;
                                            current_block = 18342783468770781838;
                                        } else {
                                            UTIL_refFilename(filenames, stdinmark.as_ptr());
                                            current_block = 4550729491376650574;
                                        }
                                    } else {
                                        current_block = 4550729491376650574;
                                    }
                                    match current_block {
                                        18342783468770781838 => {}
                                        _ => {
                                            if (*filenames).tableSize
                                                == 1 as libc::c_int as libc::c_ulong
                                                && strcmp(
                                                    *((*filenames).fileNames).offset(0 as libc::c_int as isize),
                                                    stdinmark.as_ptr(),
                                                ) == 0 && outFileName.is_null()
                                            {
                                                outFileName = stdoutmark.as_ptr();
                                            }
                                            if forceStdin == 0
                                                && UTIL_searchFileNamesTable(filenames, stdinmark.as_ptr())
                                                    != -(1 as libc::c_int) && UTIL_isConsole(stdin) != 0
                                            {
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"stdin is a console, aborting\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                            } else if (outFileName.is_null()
                                                || strcmp(outFileName, stdoutmark.as_ptr()) == 0)
                                                && UTIL_isConsole(stdout) != 0
                                                && UTIL_searchFileNamesTable(filenames, stdinmark.as_ptr())
                                                    != -(1 as libc::c_int) && forceStdout == 0
                                                && operation as libc::c_uint
                                                    != zom_decompress as libc::c_int as libc::c_uint
                                            {
                                                if g_displayLevel >= 1 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"stdout is a console, aborting\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                operationResult = 1 as libc::c_int;
                                            } else {
                                                let maxCLevel = if ultra != 0 {
                                                    ZSTD_maxCLevel()
                                                } else {
                                                    ZSTDCLI_CLEVEL_MAX
                                                };
                                                if cLevel > maxCLevel {
                                                    if g_displayLevel >= 2 as libc::c_int {
                                                        fprintf(
                                                            stderr,
                                                            b"Warning : compression level higher than max, reduced to %i \n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            maxCLevel,
                                                        );
                                                    }
                                                    cLevel = maxCLevel;
                                                }
                                                if showDefaultCParams != 0 {
                                                    if operation as libc::c_uint
                                                        == zom_decompress as libc::c_int as libc::c_uint
                                                    {
                                                        if g_displayLevel >= 1 as libc::c_int {
                                                            fprintf(
                                                                stderr,
                                                                b"error : can't use --show-default-cparams in decompression mode \n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                            );
                                                        }
                                                        operationResult = 1 as libc::c_int;
                                                        current_block = 18342783468770781838;
                                                    } else {
                                                        current_block = 1086905135267684575;
                                                    }
                                                } else {
                                                    current_block = 1086905135267684575;
                                                }
                                                match current_block {
                                                    18342783468770781838 => {}
                                                    _ => {
                                                        if !dictFileName.is_null()
                                                            && !patchFromDictFileName.is_null()
                                                        {
                                                            if g_displayLevel >= 1 as libc::c_int {
                                                                fprintf(
                                                                    stderr,
                                                                    b"error : can't use -D and --patch-from=# at the same time \n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                            }
                                                            operationResult = 1 as libc::c_int;
                                                        } else if !patchFromDictFileName.is_null()
                                                            && (*filenames).tableSize
                                                                > 1 as libc::c_int as libc::c_ulong
                                                        {
                                                            if g_displayLevel >= 1 as libc::c_int {
                                                                fprintf(
                                                                    stderr,
                                                                    b"error : can't use --patch-from=# on multiple files \n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                            }
                                                            operationResult = 1 as libc::c_int;
                                                        } else {
                                                            hasStdout = (!outFileName.is_null()
                                                                && strcmp(outFileName, stdoutmark.as_ptr()) == 0)
                                                                as libc::c_int;
                                                            if hasStdout != 0 && g_displayLevel == 2 as libc::c_int {
                                                                g_displayLevel = 1 as libc::c_int;
                                                            }
                                                            if UTIL_isConsole(stderr) == 0
                                                                && progress as libc::c_uint
                                                                    != FIO_ps_always as libc::c_int as libc::c_uint
                                                            {
                                                                progress = FIO_ps_never;
                                                            }
                                                            FIO_setProgressSetting(progress);
                                                            if hasStdout != 0 && removeSrcFile != 0 {
                                                                if g_displayLevel >= 3 as libc::c_int {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"Note: src files are not removed when output is stdout \n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                removeSrcFile = 0 as libc::c_int;
                                                            }
                                                            FIO_setRemoveSrcFile(prefs, removeSrcFile);
                                                            FIO_setHasStdoutOutput(fCtx, hasStdout);
                                                            FIO_setNbFilesTotal(
                                                                fCtx,
                                                                (*filenames).tableSize as libc::c_int,
                                                            );
                                                            FIO_determineHasStdinInput(fCtx, filenames);
                                                            FIO_setNotificationLevel(g_displayLevel);
                                                            FIO_setAllowBlockDevices(prefs, allowBlockDevices);
                                                            FIO_setPatchFromMode(
                                                                prefs,
                                                                (patchFromDictFileName != NULL as *const libc::c_char)
                                                                    as libc::c_int,
                                                            );
                                                            FIO_setMMapDict(prefs, mmapDict);
                                                            if memLimit == 0 as libc::c_int as libc::c_uint {
                                                                if compressionParams.windowLog
                                                                    == 0 as libc::c_int as libc::c_uint
                                                                {
                                                                    memLimit = (1 as libc::c_int as U32)
                                                                        << g_defaultMaxWindowLog;
                                                                } else {
                                                                    memLimit = (1 as libc::c_int as U32)
                                                                        << (compressionParams.windowLog
                                                                            & 31 as libc::c_int as libc::c_uint);
                                                                }
                                                            }
                                                            if !patchFromDictFileName.is_null() {
                                                                dictFileName = patchFromDictFileName;
                                                            }
                                                            FIO_setMemLimit(prefs, memLimit);
                                                            if operation as libc::c_uint
                                                                == zom_compress as libc::c_int as libc::c_uint
                                                            {
                                                                FIO_setCompressionType(prefs, cType);
                                                                FIO_setContentSize(prefs, contentSize);
                                                                FIO_setNbWorkers(prefs, nbWorkers as libc::c_int);
                                                                FIO_setBlockSize(prefs, blockSize as libc::c_int);
                                                                if g_overlapLog != OVERLAP_LOG_DEFAULT as libc::c_uint {
                                                                    FIO_setOverlapLog(prefs, g_overlapLog as libc::c_int);
                                                                }
                                                                FIO_setLdmFlag(prefs, ldmFlag as libc::c_uint);
                                                                FIO_setLdmHashLog(prefs, g_ldmHashLog as libc::c_int);
                                                                FIO_setLdmMinMatch(prefs, g_ldmMinMatch as libc::c_int);
                                                                if g_ldmBucketSizeLog != LDM_PARAM_DEFAULT as libc::c_uint {
                                                                    FIO_setLdmBucketSizeLog(
                                                                        prefs,
                                                                        g_ldmBucketSizeLog as libc::c_int,
                                                                    );
                                                                }
                                                                if g_ldmHashRateLog != LDM_PARAM_DEFAULT as libc::c_uint {
                                                                    FIO_setLdmHashRateLog(
                                                                        prefs,
                                                                        g_ldmHashRateLog as libc::c_int,
                                                                    );
                                                                }
                                                                FIO_setAdaptiveMode(prefs, adapt);
                                                                FIO_setUseRowMatchFinder(
                                                                    prefs,
                                                                    useRowMatchFinder as libc::c_int,
                                                                );
                                                                FIO_setAdaptMin(prefs, adaptMin);
                                                                FIO_setAdaptMax(prefs, adaptMax);
                                                                FIO_setRsyncable(prefs, rsyncable);
                                                                FIO_setStreamSrcSize(prefs, streamSrcSize);
                                                                FIO_setTargetCBlockSize(prefs, targetCBlockSize);
                                                                FIO_setSrcSizeHint(prefs, srcSizeHint);
                                                                FIO_setLiteralCompressionMode(
                                                                    prefs,
                                                                    literalCompressionMode,
                                                                );
                                                                FIO_setSparseWrite(prefs, 0 as libc::c_int);
                                                                if adaptMin > cLevel {
                                                                    cLevel = adaptMin;
                                                                }
                                                                if adaptMax < cLevel {
                                                                    cLevel = adaptMax;
                                                                }
                                                                let mut strategyBounds = ZSTD_cParam_getBounds(
                                                                    ZSTD_c_strategy,
                                                                );
                                                                if 9 as libc::c_int == strategyBounds.upperBound {} else {
                                                                    __assert_fail(
                                                                        b"ZSTD_NB_STRATEGIES == strategyBounds.upperBound\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        b"/home/peter/Dev/zstd-c2rust/programs/zstdcli.c\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        1567 as libc::c_int as libc::c_uint,
                                                                        (*::core::mem::transmute::<
                                                                            &[u8; 29],
                                                                            &[libc::c_char; 29],
                                                                        >(b"int main(int, const char **)\0"))
                                                                            .as_ptr(),
                                                                    );
                                                                }
                                                                if showDefaultCParams != 0
                                                                    || g_displayLevel >= 4 as libc::c_int
                                                                {
                                                                    let mut fileNb: size_t = 0;
                                                                    fileNb = 0 as libc::c_int as size_t;
                                                                    while fileNb < (*filenames).tableSize {
                                                                        if showDefaultCParams != 0 {
                                                                            printDefaultCParams(
                                                                                *((*filenames).fileNames).offset(fileNb as isize),
                                                                                dictFileName,
                                                                                cLevel,
                                                                            );
                                                                        }
                                                                        if g_displayLevel >= 4 as libc::c_int {
                                                                            printActualCParams(
                                                                                *((*filenames).fileNames).offset(fileNb as isize),
                                                                                dictFileName,
                                                                                cLevel,
                                                                                &mut compressionParams,
                                                                            );
                                                                        }
                                                                        fileNb = fileNb.wrapping_add(1);
                                                                    }
                                                                }
                                                                if g_displayLevel >= 4 as libc::c_int {
                                                                    FIO_displayCompressionParameters(prefs);
                                                                }
                                                                if (*filenames).tableSize
                                                                    == 1 as libc::c_int as libc::c_ulong
                                                                    && !outFileName.is_null()
                                                                {
                                                                    operationResult = FIO_compressFilename(
                                                                        fCtx,
                                                                        prefs,
                                                                        outFileName,
                                                                        *((*filenames).fileNames).offset(0 as libc::c_int as isize),
                                                                        dictFileName,
                                                                        cLevel,
                                                                        compressionParams,
                                                                    );
                                                                } else {
                                                                    operationResult = FIO_compressMultipleFilenames(
                                                                        fCtx,
                                                                        prefs,
                                                                        (*filenames).fileNames,
                                                                        outMirroredDirName,
                                                                        outDirName,
                                                                        outFileName,
                                                                        suffix,
                                                                        dictFileName,
                                                                        cLevel,
                                                                        compressionParams,
                                                                    );
                                                                }
                                                            } else if (*filenames).tableSize
                                                                == 1 as libc::c_int as libc::c_ulong
                                                                && !outFileName.is_null()
                                                            {
                                                                operationResult = FIO_decompressFilename(
                                                                    fCtx,
                                                                    prefs,
                                                                    outFileName,
                                                                    *((*filenames).fileNames).offset(0 as libc::c_int as isize),
                                                                    dictFileName,
                                                                );
                                                            } else {
                                                                operationResult = FIO_decompressMultipleFilenames(
                                                                    fCtx,
                                                                    prefs,
                                                                    (*filenames).fileNames,
                                                                    outMirroredDirName,
                                                                    outDirName,
                                                                    outFileName,
                                                                    dictFileName,
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    FIO_freePreferences(prefs);
    FIO_freeContext(fCtx);
    if main_pause != 0 {
        waitEnter();
    }
    UTIL_freeFileNamesTable(filenames);
    UTIL_freeFileNamesTable(file_of_names);
    return operationResult;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
