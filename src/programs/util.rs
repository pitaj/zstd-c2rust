use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
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
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint64_t = __uint64_t;
pub type U64 = uint64_t;
pub type stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_HumanReadableSize_t {
    pub value: libc::c_double,
    pub precision: libc::c_int,
    pub suffix: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union charunion {
    pub chr: *mut libc::c_char,
    pub cchr: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const libc::c_char,
    pub buf: *mut libc::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const EOF: libc::c_int = -(1 as libc::c_int);
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const UTIME_NOW: libc::c_long = ((1 as libc::c_long) << 30 as libc::c_int)
    - 1 as libc::c_long;
pub const UTIL_FILESIZE_UNKNOWN: libc::c_int = -(1 as libc::c_int);
pub const _SC_NPROCESSORS_ONLN_0: libc::c_int = _SC_NPROCESSORS_ONLN as libc::c_int;
pub const PATH_SEP: libc::c_int = '/' as i32;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EEXIST: libc::c_int = 17 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const AT_FDCWD: libc::c_int = -(100 as libc::c_int);
static mut g_traceDepth: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut g_traceFileStat: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn UTIL_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut newptr = realloc(ptr, size);
    if !newptr.is_null() {
        return newptr;
    }
    free(ptr);
    return NULL as *mut libc::c_void;
}
#[no_mangle]
pub static mut g_utilDisplayLevel: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UTIL_requireUserConfirmation(
    mut prompt: *const libc::c_char,
    mut abortMsg: *const libc::c_char,
    mut acceptableLetters: *const libc::c_char,
    mut hasStdinInput: libc::c_int,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    if hasStdinInput != 0 {
        fprintf(
            stderr,
            b"stdin is an input - not proceeding.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, prompt);
    ch = getchar();
    result = 0 as libc::c_int;
    if (strchr(acceptableLetters, ch)).is_null() {
        fprintf(stderr, b"%s \n\0" as *const u8 as *const libc::c_char, abortMsg);
        result = 1 as libc::c_int;
    }
    while ch != EOF && ch != '\n' as i32 {
        ch = getchar();
    }
    return result;
}
pub const LIST_SIZE_INCREASE: libc::c_int = 8 as libc::c_int * 1024 as libc::c_int;
pub const MAX_FILE_OF_FILE_NAMES_SIZE: libc::c_int = ((1 as libc::c_int)
    << 20 as libc::c_int) * 50 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn UTIL_traceFileStat() {
    g_traceFileStat = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fstat(
    fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut statbuf: *mut stat_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_stat(%d, %s)\0" as *const u8 as *const libc::c_char,
            fd,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    if fd >= 0 as libc::c_int {
        ret = (fstat(fd, statbuf) == 0) as libc::c_int;
    } else {
        ret = (stat(filename, statbuf) == 0) as libc::c_int;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_stat(
    mut filename: *const libc::c_char,
    mut statbuf: *mut stat_t,
) -> libc::c_int {
    return UTIL_fstat(-(1 as libc::c_int), filename, statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isRegularFile(
    mut infilename: *const libc::c_char,
) -> libc::c_int {
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isRegularFile(%s)\0" as *const u8 as *const libc::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    ret = (UTIL_stat(infilename, &mut statbuf) != 0
        && UTIL_isRegularFileStat(&mut statbuf) != 0) as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isRegularFileStat(
    mut statbuf: *const stat_t,
) -> libc::c_int {
    return (((*statbuf).st_mode & __S_IFMT as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_chmod(
    mut filename: *const libc::c_char,
    mut statbuf: *const stat_t,
    mut permissions: mode_t,
) -> libc::c_int {
    return UTIL_fchmod(-(1 as libc::c_int), filename, statbuf, permissions);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fchmod(
    fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut statbuf: *const stat_t,
    mut permissions: mode_t,
) -> libc::c_int {
    let mut localStatBuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_chmod(%s, %#4o)\0" as *const u8 as *const libc::c_char,
            filename,
            permissions,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    if statbuf.is_null() {
        if UTIL_fstat(fd, filename, &mut localStatBuf) == 0 {
            if g_traceFileStat != 0 {
                g_traceDepth -= 1;
                fprintf(
                    stderr,
                    b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                    g_traceDepth,
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            return 0 as libc::c_int;
        }
        statbuf = &mut localStatBuf;
    }
    if UTIL_isRegularFileStat(statbuf) == 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        return 0 as libc::c_int;
    }
    if fd >= 0 as libc::c_int {
        let mut ret: libc::c_int = 0;
        if g_traceFileStat != 0 {
            fprintf(
                stderr,
                b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"fchmod\0" as *const u8 as *const libc::c_char);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            g_traceDepth += 1;
        }
        ret = fchmod(fd, permissions);
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                ret,
            );
        }
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                ret,
            );
        }
        return ret;
    } else {
        let mut ret_0: libc::c_int = 0;
        if g_traceFileStat != 0 {
            fprintf(
                stderr,
                b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"chmod\0" as *const u8 as *const libc::c_char);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            g_traceDepth += 1;
        }
        ret_0 = chmod(filename, permissions);
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                ret_0,
            );
        }
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                ret_0,
            );
        }
        return ret_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_utime(
    mut filename: *const libc::c_char,
    mut statbuf: *const stat_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_utime(%s)\0" as *const u8 as *const libc::c_char,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    let mut timebuf: [timespec; 2] = [
        {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: UTIME_NOW,
            };
            init
        },
        timespec { tv_sec: 0, tv_nsec: 0 },
    ];
    timebuf[1 as libc::c_int as usize] = (*statbuf).st_mtim;
    ret = utimensat(
        AT_FDCWD,
        filename,
        timebuf.as_mut_ptr() as *const timespec,
        0 as libc::c_int,
    );
    errno = 0 as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_setFileStat(
    mut filename: *const libc::c_char,
    mut statbuf: *const stat_t,
) -> libc::c_int {
    return UTIL_setFDStat(-(1 as libc::c_int), filename, statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_setFDStat(
    fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut statbuf: *const stat_t,
) -> libc::c_int {
    let mut res = 0 as libc::c_int;
    let mut curStatBuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_setFileStat(%d, %s)\0" as *const u8 as *const libc::c_char,
            fd,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    if UTIL_fstat(fd, filename, &mut curStatBuf) == 0
        || UTIL_isRegularFileStat(&mut curStatBuf) == 0
    {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        return -(1 as libc::c_int);
    }
    if fd >= 0 as libc::c_int {
        res += fchown(fd, -(1 as libc::c_int) as __uid_t, (*statbuf).st_gid);
    } else {
        res += chown(filename, -(1 as libc::c_int) as __uid_t, (*statbuf).st_gid);
    }
    res
        += UTIL_fchmod(
            fd,
            filename,
            &mut curStatBuf,
            (*statbuf).st_mode & 0o777 as libc::c_int as libc::c_uint,
        );
    if fd >= 0 as libc::c_int {
        res += fchown(fd, (*statbuf).st_uid, -(1 as libc::c_int) as __gid_t);
    } else {
        res += chown(filename, (*statbuf).st_uid, -(1 as libc::c_int) as __gid_t);
    }
    errno = 0 as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            -res,
        );
    }
    return -res;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isDirectory(
    mut infilename: *const libc::c_char,
) -> libc::c_int {
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isDirectory(%s)\0" as *const u8 as *const libc::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    ret = (UTIL_stat(infilename, &mut statbuf) != 0
        && UTIL_isDirectoryStat(&mut statbuf) != 0) as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isDirectoryStat(
    mut statbuf: *const stat_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"UTIL_isDirectoryStat()\0" as *const u8 as *const libc::c_char);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    ret = (((*statbuf).st_mode & __S_IFMT as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_compareStr(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return strcmp(*(p1 as *const *mut libc::c_char), *(p2 as *const *mut libc::c_char));
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isSameFile(
    mut fName1: *const libc::c_char,
    mut fName2: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if !fName1.is_null() {} else {
        __assert_fail(
            b"fName1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            365 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int UTIL_isSameFile(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    if !fName2.is_null() {} else {
        __assert_fail(
            b"fName2 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            365 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int UTIL_isSameFile(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isSameFile(%s, %s)\0" as *const u8 as *const libc::c_char,
            fName1,
            fName2,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    let mut file1Stat = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut file2Stat = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    ret = (UTIL_stat(fName1, &mut file1Stat) != 0
        && UTIL_stat(fName2, &mut file2Stat) != 0
        && UTIL_isSameFileStat(fName1, fName2, &mut file1Stat, &mut file2Stat) != 0)
        as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isSameFileStat(
    mut fName1: *const libc::c_char,
    mut fName2: *const libc::c_char,
    mut file1Stat: *const stat_t,
    mut file2Stat: *const stat_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if !fName1.is_null() {} else {
        __assert_fail(
            b"fName1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            390 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"int UTIL_isSameFileStat(const char *, const char *, const stat_t *, const stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !fName2.is_null() {} else {
        __assert_fail(
            b"fName2 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            390 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"int UTIL_isSameFileStat(const char *, const char *, const stat_t *, const stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isSameFileStat(%s, %s)\0" as *const u8 as *const libc::c_char,
            fName1,
            fName2,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    ret = ((*file1Stat).st_dev == (*file2Stat).st_dev
        && (*file1Stat).st_ino == (*file2Stat).st_ino) as libc::c_int;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isFIFO(
    mut infilename: *const libc::c_char,
) -> libc::c_int {
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isFIFO(%s)\0" as *const u8 as *const libc::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(infilename, &mut statbuf) != 0 && UTIL_isFIFOStat(&mut statbuf) != 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        return 1 as libc::c_int;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isFIFOStat(mut statbuf: *const stat_t) -> libc::c_int {
    if (*statbuf).st_mode & __S_IFMT as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isBlockDevStat(mut statbuf: *const stat_t) -> libc::c_int {
    if (*statbuf).st_mode & __S_IFMT as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isLink(
    mut infilename: *const libc::c_char,
) -> libc::c_int {
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isLink(%s)\0" as *const u8 as *const libc::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let r = lstat(infilename, &mut statbuf);
    if r == 0
        && statbuf.st_mode & __S_IFMT as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
    {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        return 1 as libc::c_int;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
static mut g_fakeStdinIsConsole: libc::c_int = 0 as libc::c_int;
static mut g_fakeStderrIsConsole: libc::c_int = 0 as libc::c_int;
static mut g_fakeStdoutIsConsole: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn UTIL_isConsole(mut file: *mut FILE) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isConsole(%d)\0" as *const u8 as *const libc::c_char,
            fileno(file),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    if file == stdin && g_fakeStdinIsConsole != 0 {
        ret = 1 as libc::c_int;
    } else if file == stderr && g_fakeStderrIsConsole != 0 {
        ret = 1 as libc::c_int;
    } else if file == stdout && g_fakeStdoutIsConsole != 0 {
        ret = 1 as libc::c_int;
    } else {
        ret = isatty(fileno(file));
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fakeStdinIsConsole() {
    g_fakeStdinIsConsole = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fakeStdoutIsConsole() {
    g_fakeStdoutIsConsole = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fakeStderrIsConsole() {
    g_fakeStderrIsConsole = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getFileSize(mut infilename: *const libc::c_char) -> U64 {
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_getFileSize(%s)\0" as *const u8 as *const libc::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    if UTIL_stat(infilename, &mut statbuf) == 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        return UTIL_FILESIZE_UNKNOWN as U64;
    }
    let size = UTIL_getFileSizeStat(&mut statbuf);
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            size as libc::c_int,
        );
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getFileSizeStat(mut statbuf: *const stat_t) -> U64 {
    if UTIL_isRegularFileStat(statbuf) == 0 {
        return UTIL_FILESIZE_UNKNOWN as U64;
    }
    if !((*statbuf).st_mode & __S_IFMT as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        return UTIL_FILESIZE_UNKNOWN as U64;
    }
    return (*statbuf).st_size as U64;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_makeHumanReadableSize(
    mut size: U64,
) -> UTIL_HumanReadableSize_t {
    let mut hrs = UTIL_HumanReadableSize_t {
        value: 0.,
        precision: 0,
        suffix: 0 as *const libc::c_char,
    };
    if g_utilDisplayLevel > 3 as libc::c_int {
        if size as libc::c_ulonglong >= (1 as libc::c_ulonglong) << 53 as libc::c_int {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 20 as libc::c_int) as libc::c_double;
            hrs.suffix = b" MiB\0" as *const u8 as *const libc::c_char;
            hrs.precision = 2 as libc::c_int;
        } else {
            hrs.value = size as libc::c_double;
            hrs.suffix = b" B\0" as *const u8 as *const libc::c_char;
            hrs.precision = 0 as libc::c_int;
        }
    } else {
        if size as libc::c_ulonglong >= (1 as libc::c_ulonglong) << 60 as libc::c_int {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 60 as libc::c_int) as libc::c_double;
            hrs.suffix = b" EiB\0" as *const u8 as *const libc::c_char;
        } else if size as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 50 as libc::c_int
        {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 50 as libc::c_int) as libc::c_double;
            hrs.suffix = b" PiB\0" as *const u8 as *const libc::c_char;
        } else if size as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 40 as libc::c_int
        {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 40 as libc::c_int) as libc::c_double;
            hrs.suffix = b" TiB\0" as *const u8 as *const libc::c_char;
        } else if size as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 30 as libc::c_int
        {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 30 as libc::c_int) as libc::c_double;
            hrs.suffix = b" GiB\0" as *const u8 as *const libc::c_char;
        } else if size as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 20 as libc::c_int
        {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 20 as libc::c_int) as libc::c_double;
            hrs.suffix = b" MiB\0" as *const u8 as *const libc::c_char;
        } else if size as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 10 as libc::c_int
        {
            hrs
                .value = size as libc::c_double
                / ((1 as libc::c_ulonglong) << 10 as libc::c_int) as libc::c_double;
            hrs.suffix = b" KiB\0" as *const u8 as *const libc::c_char;
        } else {
            hrs.value = size as libc::c_double;
            hrs.suffix = b" B\0" as *const u8 as *const libc::c_char;
        }
        if hrs.value >= 100 as libc::c_int as libc::c_double || hrs.value as U64 == size
        {
            hrs.precision = 0 as libc::c_int;
        } else if hrs.value >= 10 as libc::c_int as libc::c_double {
            hrs.precision = 1 as libc::c_int;
        } else if hrs.value > 1 as libc::c_int as libc::c_double {
            hrs.precision = 2 as libc::c_int;
        } else {
            hrs.precision = 3 as libc::c_int;
        }
    }
    return hrs;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getTotalFileSize(
    mut fileNamesTable: *const *const libc::c_char,
    mut nbFiles: libc::c_uint,
) -> U64 {
    let mut total = 0 as libc::c_int as U64;
    let mut n: libc::c_uint = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_getTotalFileSize(%u)\0" as *const u8 as *const libc::c_char,
            nbFiles,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        g_traceDepth += 1;
    }
    n = 0 as libc::c_int as libc::c_uint;
    while n < nbFiles {
        let size = UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if size == UTIL_FILESIZE_UNKNOWN as U64 {
            if g_traceFileStat != 0 {
                g_traceDepth -= 1;
                fprintf(
                    stderr,
                    b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
                    g_traceDepth,
                    b"\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            }
            return UTIL_FILESIZE_UNKNOWN as U64;
        }
        total = (total as libc::c_ulong).wrapping_add(size) as U64 as U64;
        n = n.wrapping_add(1);
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const libc::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const libc::c_char,
            total as libc::c_int,
        );
    }
    return total;
}
unsafe extern "C" fn readLineFromFile(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut file: *mut FILE,
) -> size_t {
    if feof(file) == 0 {} else {
        __assert_fail(
            b"!feof(file)\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            611 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"size_t readLineFromFile(char *, size_t, FILE *)\0"))
                .as_ptr(),
        );
    }
    if (fgets(buf, len as libc::c_int, file)).is_null() {
        return 0 as libc::c_int as size_t;
    }
    let mut linelen = strlen(buf);
    if strlen(buf) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if *buf.offset(linelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\n' as i32
    {
        linelen = linelen.wrapping_sub(1);
    }
    *buf.offset(linelen as isize) = '\0' as i32 as libc::c_char;
    return linelen.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn readLinesFromFile(
    mut dst: *mut libc::c_void,
    mut dstCapacity: size_t,
    mut inputFileName: *const libc::c_char,
) -> libc::c_int {
    let mut nbFiles = 0 as libc::c_int;
    let mut pos = 0 as libc::c_int as size_t;
    let buf = dst as *mut libc::c_char;
    let inputFile = fopen(inputFileName, b"r\0" as *const u8 as *const libc::c_char);
    if !dst.is_null() {} else {
        __assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            636 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"int readLinesFromFile(void *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    if inputFile.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            perror(b"zstd:util:readLinesFromFile\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    while feof(inputFile) == 0 {
        let lineLength = readLineFromFile(
            buf.offset(pos as isize),
            dstCapacity.wrapping_sub(pos),
            inputFile,
        );
        if lineLength == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if pos.wrapping_add(lineLength) <= dstCapacity {} else {
            __assert_fail(
                b"pos + lineLength <= dstCapacity\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                646 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"int readLinesFromFile(void *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
        pos = (pos as libc::c_ulong).wrapping_add(lineLength) as size_t as size_t;
        nbFiles += 1;
    }
    if !(fclose(inputFile) == 0 as libc::c_int) {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                651 as libc::c_int,
                b"fclose(inputFile) == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    return nbFiles;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_createFileNamesTable_fromFileName(
    mut inputFileName: *const libc::c_char,
) -> *mut FileNamesTable {
    let mut nbFiles = 0 as libc::c_int as size_t;
    let mut buf = 0 as *mut libc::c_char;
    let mut bufSize: size_t = 0;
    let mut pos = 0 as libc::c_int as size_t;
    let mut statbuf = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(inputFileName, &mut statbuf) == 0
        || UTIL_isRegularFileStat(&mut statbuf) == 0
    {
        return NULL as *mut FileNamesTable;
    }
    let inputFileSize = UTIL_getFileSizeStat(&mut statbuf);
    if inputFileSize > MAX_FILE_OF_FILE_NAMES_SIZE as libc::c_ulong {
        return NULL as *mut FileNamesTable;
    }
    bufSize = inputFileSize.wrapping_add(1 as libc::c_int as libc::c_ulong);
    buf = malloc(bufSize) as *mut libc::c_char;
    if buf.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                676 as libc::c_int,
                b"buf != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    let ret_nbFiles = readLinesFromFile(
        buf as *mut libc::c_void,
        bufSize,
        inputFileName,
    );
    if ret_nbFiles <= 0 as libc::c_int {
        free(buf as *mut libc::c_void);
        return NULL as *mut FileNamesTable;
    }
    nbFiles = ret_nbFiles as size_t;
    let mut filenamesTable = malloc(
        nbFiles
            .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    if filenamesTable.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                688 as libc::c_int,
                b"filenamesTable != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    let mut fnb: size_t = 0;
    fnb = 0 as libc::c_int as size_t;
    pos = 0 as libc::c_int as size_t;
    while fnb < nbFiles {
        let ref mut fresh0 = *filenamesTable.offset(fnb as isize);
        *fresh0 = buf.offset(pos as isize);
        pos = (pos as libc::c_ulong)
            .wrapping_add(
                (strlen(buf.offset(pos as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        fnb = fnb.wrapping_add(1);
    }
    if pos <= bufSize {} else {
        __assert_fail(
            b"pos <= bufSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            695 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"FileNamesTable *UTIL_createFileNamesTable_fromFileName(const char *)\0"))
                .as_ptr(),
        );
    }
    return UTIL_assembleFileNamesTable(filenamesTable, nbFiles, buf);
}
unsafe extern "C" fn UTIL_assembleFileNamesTable2(
    mut filenames: *mut *const libc::c_char,
    mut tableSize: size_t,
    mut tableCapacity: size_t,
    mut buf: *mut libc::c_char,
) -> *mut FileNamesTable {
    let table = malloc(::core::mem::size_of::<FileNamesTable>() as libc::c_ulong)
        as *mut FileNamesTable;
    if table.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                705 as libc::c_int,
                b"table != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    (*table).fileNames = filenames;
    (*table).buf = buf;
    (*table).tableSize = tableSize;
    (*table).tableCapacity = tableCapacity;
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_assembleFileNamesTable(
    mut filenames: *mut *const libc::c_char,
    mut tableSize: size_t,
    mut buf: *mut libc::c_char,
) -> *mut FileNamesTable {
    return UTIL_assembleFileNamesTable2(filenames, tableSize, tableSize, buf);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_freeFileNamesTable(mut table: *mut FileNamesTable) {
    if table.is_null() {
        return;
    }
    free((*table).fileNames as *mut libc::c_void);
    free((*table).buf as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_allocateFileNamesTable(
    mut tableSize: size_t,
) -> *mut FileNamesTable {
    let fnTable = malloc(
        tableSize
            .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    let mut fnt = 0 as *mut FileNamesTable;
    if fnTable.is_null() {
        return NULL as *mut FileNamesTable;
    }
    fnt = UTIL_assembleFileNamesTable(fnTable, tableSize, NULL as *mut libc::c_char);
    (*fnt).tableSize = 0 as libc::c_int as size_t;
    return fnt;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_searchFileNamesTable(
    mut table: *mut FileNamesTable,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*table).tableSize {
        if strcmp(*((*table).fileNames).offset(i as isize), name) == 0 {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_refFilename(
    mut fnt: *mut FileNamesTable,
    mut filename: *const libc::c_char,
) {
    if (*fnt).tableSize < (*fnt).tableCapacity {} else {
        __assert_fail(
            b"fnt->tableSize < fnt->tableCapacity\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            749 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void UTIL_refFilename(FileNamesTable *, const char *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh1 = *((*fnt).fileNames).offset((*fnt).tableSize as isize);
    *fresh1 = filename;
    (*fnt).tableSize = ((*fnt).tableSize).wrapping_add(1);
}
unsafe extern "C" fn getTotalTableSize(mut table: *mut FileNamesTable) -> size_t {
    let mut fnb = 0 as libc::c_int as size_t;
    let mut totalSize = 0 as libc::c_int as size_t;
    fnb = 0 as libc::c_int as size_t;
    while fnb < (*table).tableSize
        && !(*((*table).fileNames).offset(fnb as isize)).is_null()
    {
        totalSize = (totalSize as libc::c_ulong)
            .wrapping_add(
                (strlen(*((*table).fileNames).offset(fnb as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        fnb = fnb.wrapping_add(1);
    }
    return totalSize;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_mergeFileNamesTable(
    mut table1: *mut FileNamesTable,
    mut table2: *mut FileNamesTable,
) -> *mut FileNamesTable {
    let mut newTableIdx = 0 as libc::c_int as libc::c_uint;
    let mut pos = 0 as libc::c_int as size_t;
    let mut newTotalTableSize: size_t = 0;
    let mut buf = 0 as *mut libc::c_char;
    let newTable = UTIL_assembleFileNamesTable(
        NULL as *mut *const libc::c_char,
        0 as libc::c_int as size_t,
        NULL as *mut libc::c_char,
    );
    if newTable.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                772 as libc::c_int,
                b"newTable != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    newTotalTableSize = (getTotalTableSize(table1))
        .wrapping_add(getTotalTableSize(table2));
    buf = calloc(
        newTotalTableSize,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    if buf.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                777 as libc::c_int,
                b"buf != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    (*newTable).buf = buf;
    (*newTable).tableSize = ((*table1).tableSize).wrapping_add((*table2).tableSize);
    (*newTable)
        .fileNames = calloc(
        (*newTable).tableSize,
        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    if ((*newTable).fileNames).is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                782 as libc::c_int,
                b"newTable->fileNames != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    let mut idx1: libc::c_uint = 0;
    idx1 = 0 as libc::c_int as libc::c_uint;
    while (idx1 as libc::c_ulong) < (*table1).tableSize
        && !(*((*table1).fileNames).offset(idx1 as isize)).is_null()
        && pos < newTotalTableSize
    {
        let curLen = strlen(*((*table1).fileNames).offset(idx1 as isize));
        memcpy(
            buf.offset(pos as isize) as *mut libc::c_void,
            *((*table1).fileNames).offset(idx1 as isize) as *const libc::c_void,
            curLen,
        );
        if newTableIdx as libc::c_ulong <= (*newTable).tableSize {} else {
            __assert_fail(
                b"newTableIdx <= newTable->tableSize\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                788 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"FileNamesTable *UTIL_mergeFileNamesTable(FileNamesTable *, FileNamesTable *)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh2 = *((*newTable).fileNames).offset(newTableIdx as isize);
        *fresh2 = buf.offset(pos as isize);
        pos = (pos as libc::c_ulong)
            .wrapping_add(curLen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        idx1 = idx1.wrapping_add(1);
        newTableIdx = newTableIdx.wrapping_add(1);
    }
    let mut idx2: libc::c_uint = 0;
    idx2 = 0 as libc::c_int as libc::c_uint;
    while (idx2 as libc::c_ulong) < (*table2).tableSize
        && !(*((*table2).fileNames).offset(idx2 as isize)).is_null()
        && pos < newTotalTableSize
    {
        let curLen_0 = strlen(*((*table2).fileNames).offset(idx2 as isize));
        memcpy(
            buf.offset(pos as isize) as *mut libc::c_void,
            *((*table2).fileNames).offset(idx2 as isize) as *const libc::c_void,
            curLen_0,
        );
        if (newTableIdx as libc::c_ulong) < (*newTable).tableSize {} else {
            __assert_fail(
                b"newTableIdx < newTable->tableSize\0" as *const u8
                    as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                797 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"FileNamesTable *UTIL_mergeFileNamesTable(FileNamesTable *, FileNamesTable *)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh3 = *((*newTable).fileNames).offset(newTableIdx as isize);
        *fresh3 = buf.offset(pos as isize);
        pos = (pos as libc::c_ulong)
            .wrapping_add(curLen_0.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        idx2 = idx2.wrapping_add(1);
        newTableIdx = newTableIdx.wrapping_add(1);
    }
    if pos <= newTotalTableSize {} else {
        __assert_fail(
            b"pos <= newTotalTableSize\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            801 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"FileNamesTable *UTIL_mergeFileNamesTable(FileNamesTable *, FileNamesTable *)\0",
            ))
                .as_ptr(),
        );
    }
    (*newTable).tableSize = newTableIdx as size_t;
    UTIL_freeFileNamesTable(table1);
    UTIL_freeFileNamesTable(table2);
    return newTable;
}
unsafe extern "C" fn UTIL_prepareFileList(
    mut dirName: *const libc::c_char,
    mut bufStart: *mut *mut libc::c_char,
    mut pos: *mut size_t,
    mut bufEnd: *mut *mut libc::c_char,
    mut followLinks: libc::c_int,
) -> libc::c_int {
    let mut dir = 0 as *mut DIR;
    let mut entry = 0 as *mut dirent;
    let mut dirLength: size_t = 0;
    let mut nbFiles = 0 as libc::c_int;
    dir = opendir(dirName);
    if dir.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Cannot open directory '%s': %s\n\0" as *const u8
                    as *const libc::c_char,
                dirName,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    dirLength = strlen(dirName);
    errno = 0 as libc::c_int;
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        let mut path = 0 as *mut libc::c_char;
        let mut fnameLength: size_t = 0;
        let mut pathLength: size_t = 0;
        if strcmp(
            ((*entry).d_name).as_mut_ptr(),
            b"..\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                ((*entry).d_name).as_mut_ptr(),
                b".\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            continue;
        }
        fnameLength = strlen(((*entry).d_name).as_mut_ptr());
        path = malloc(
            dirLength
                .wrapping_add(fnameLength)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if path.is_null() {
            closedir(dir);
            return 0 as libc::c_int;
        }
        memcpy(path as *mut libc::c_void, dirName as *const libc::c_void, dirLength);
        *path.offset(dirLength as isize) = '/' as i32 as libc::c_char;
        memcpy(
            path.offset(dirLength as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ((*entry).d_name).as_mut_ptr() as *const libc::c_void,
            fnameLength,
        );
        pathLength = dirLength
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(fnameLength);
        *path.offset(pathLength as isize) = 0 as libc::c_int as libc::c_char;
        if followLinks == 0 && UTIL_isLink(path) != 0 {
            if g_utilDisplayLevel >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"Warning : %s is a symbolic link, ignoring\n\0" as *const u8
                        as *const libc::c_char,
                    path,
                );
            }
            free(path as *mut libc::c_void);
        } else {
            if UTIL_isDirectory(path) != 0 {
                nbFiles
                    += UTIL_prepareFileList(path, bufStart, pos, bufEnd, followLinks);
                if (*bufStart).is_null() {
                    free(path as *mut libc::c_void);
                    closedir(dir);
                    return 0 as libc::c_int;
                }
            } else {
                if (*bufStart).offset(*pos as isize).offset(pathLength as isize)
                    >= *bufEnd
                {
                    let mut newListSize = (*bufEnd).offset_from(*bufStart)
                        as libc::c_long + LIST_SIZE_INCREASE as libc::c_long;
                    if newListSize >= 0 as libc::c_int as libc::c_long {} else {
                        __assert_fail(
                            b"newListSize >= 0\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                                as *const libc::c_char,
                            919 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int UTIL_prepareFileList(const char *, char **, size_t *, char **, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *bufStart = UTIL_realloc(
                        *bufStart as *mut libc::c_void,
                        newListSize as size_t,
                    ) as *mut libc::c_char;
                    if !(*bufStart).is_null() {
                        *bufEnd = (*bufStart).offset(newListSize as isize);
                    } else {
                        free(path as *mut libc::c_void);
                        closedir(dir);
                        return 0 as libc::c_int;
                    }
                }
                if (*bufStart).offset(*pos as isize).offset(pathLength as isize)
                    < *bufEnd
                {
                    memcpy(
                        (*bufStart).offset(*pos as isize) as *mut libc::c_void,
                        path as *const libc::c_void,
                        pathLength.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    *pos = (*pos as libc::c_ulong)
                        .wrapping_add(
                            pathLength.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    nbFiles += 1;
                }
            }
            free(path as *mut libc::c_void);
            errno = 0 as libc::c_int;
        }
    }
    if errno != 0 as libc::c_int {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"readdir(%s) error: %s \n\0" as *const u8 as *const libc::c_char,
                dirName,
                strerror(*__errno_location()),
            );
        }
        free(*bufStart as *mut libc::c_void);
        *bufStart = NULL as *mut libc::c_char;
    }
    closedir(dir);
    return nbFiles;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isCompressedFile(
    mut inputName: *const libc::c_char,
    mut extensionList: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ext = UTIL_getFileExtension(inputName);
    while !(*extensionList).is_null() {
        let isCompressedExtension = strcmp(ext, *extensionList);
        if isCompressedExtension == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        extensionList = extensionList.offset(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getFileExtension(
    mut infilename: *const libc::c_char,
) -> *const libc::c_char {
    let mut extension: *const libc::c_char = strrchr(infilename, '.' as i32);
    if extension.is_null() || extension == infilename {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return extension;
}
unsafe extern "C" fn pathnameHas2Dots(mut pathname: *const libc::c_char) -> libc::c_int {
    let mut needle = pathname;
    loop {
        needle = strstr(needle, b"..\0" as *const u8 as *const libc::c_char);
        if needle.is_null() {
            return 0 as libc::c_int;
        }
        if (needle == pathname
            || *needle.offset(-(1 as libc::c_int) as isize) as libc::c_int == PATH_SEP)
            && (*needle.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
                || *needle.offset(2 as libc::c_int as isize) as libc::c_int == PATH_SEP)
        {
            return 1 as libc::c_int;
        }
        needle = needle.offset(1);
    };
}
unsafe extern "C" fn isFileNameValidForMirroredOutput(
    mut filename: *const libc::c_char,
) -> libc::c_int {
    return (pathnameHas2Dots(filename) == 0) as libc::c_int;
}
pub const DIR_DEFAULT_MODE: libc::c_int = 0o755 as libc::c_int;
unsafe extern "C" fn getDirMode(mut dirName: *const libc::c_char) -> mode_t {
    let mut st = stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(dirName, &mut st) == 0 {
        fprintf(
            stderr,
            b"zstd: failed to get DIR stats %s: %s\n\0" as *const u8
                as *const libc::c_char,
            dirName,
            strerror(*__errno_location()),
        );
        return DIR_DEFAULT_MODE as mode_t;
    }
    if UTIL_isDirectoryStat(&mut st) == 0 {
        fprintf(
            stderr,
            b"zstd: expected directory: %s\n\0" as *const u8 as *const libc::c_char,
            dirName,
        );
        return DIR_DEFAULT_MODE as mode_t;
    }
    return st.st_mode;
}
unsafe extern "C" fn makeDir(
    mut dir: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ret = mkdir(dir, mode);
    if ret != 0 as libc::c_int {
        if errno == EEXIST {
            return 0 as libc::c_int;
        }
        fprintf(
            stderr,
            b"zstd: failed to create DIR %s: %s\n\0" as *const u8 as *const libc::c_char,
            dir,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
unsafe extern "C" fn convertPathnameToDirName(mut pathname: *mut libc::c_char) {
    let mut len = 0 as libc::c_int as size_t;
    let mut pos = NULL as *mut libc::c_char;
    if !pathname.is_null() {} else {
        __assert_fail(
            b"pathname != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            1047 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void convertPathnameToDirName(char *)\0"))
                .as_ptr(),
        );
    }
    len = strlen(pathname);
    if len > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"len > 0\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            1051 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void convertPathnameToDirName(char *)\0"))
                .as_ptr(),
        );
    }
    while *pathname.offset(len as isize) as libc::c_int == PATH_SEP {
        *pathname.offset(len as isize) = '\0' as i32 as libc::c_char;
        len = len.wrapping_sub(1);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    pos = strrchr(pathname, PATH_SEP);
    if pos.is_null() {
        *pathname.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
        *pathname.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        *pos = '\0' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn trimLeadingRootChar(
    mut pathname: *const libc::c_char,
) -> *const libc::c_char {
    if !pathname.is_null() {} else {
        __assert_fail(
            b"pathname != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            1075 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"const char *trimLeadingRootChar(const char *)\0"))
                .as_ptr(),
        );
    }
    if *pathname.offset(0 as libc::c_int as isize) as libc::c_int == PATH_SEP {
        return pathname.offset(1 as libc::c_int as isize);
    }
    return pathname;
}
unsafe extern "C" fn trimLeadingCurrentDirConst(
    mut pathname: *const libc::c_char,
) -> *const libc::c_char {
    if !pathname.is_null() {} else {
        __assert_fail(
            b"pathname != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            1084 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"const char *trimLeadingCurrentDirConst(const char *)\0"))
                .as_ptr(),
        );
    }
    if *pathname.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *pathname.offset(1 as libc::c_int as isize) as libc::c_int == PATH_SEP
    {
        return pathname.offset(2 as libc::c_int as isize);
    }
    return pathname;
}
unsafe extern "C" fn trimLeadingCurrentDir(
    mut pathname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr = charunion {
        chr: 0 as *mut libc::c_char,
    };
    ptr.cchr = trimLeadingCurrentDirConst(pathname);
    return ptr.chr;
}
unsafe extern "C" fn trimPath(mut pathname: *const libc::c_char) -> *const libc::c_char {
    return trimLeadingRootChar(trimLeadingCurrentDirConst(pathname));
}
unsafe extern "C" fn mallocAndJoin2Dir(
    mut dir1: *const libc::c_char,
    mut dir2: *const libc::c_char,
) -> *mut libc::c_char {
    if !dir1.is_null() && !dir2.is_null() {} else {
        __assert_fail(
            b"dir1 != NULL && dir2 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                as *const libc::c_char,
            1111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"char *mallocAndJoin2Dir(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    let dir1Size = strlen(dir1);
    let dir2Size = strlen(dir2);
    let mut outDirBuffer = 0 as *mut libc::c_char;
    let mut buffer = 0 as *mut libc::c_char;
    outDirBuffer = malloc(
        dir1Size.wrapping_add(dir2Size).wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if outDirBuffer.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                1117 as libc::c_int,
                b"outDirBuffer != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    memcpy(outDirBuffer as *mut libc::c_void, dir1 as *const libc::c_void, dir1Size);
    *outDirBuffer.offset(dir1Size as isize) = '\0' as i32 as libc::c_char;
    if *dir2.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        return outDirBuffer;
    }
    buffer = outDirBuffer.offset(dir1Size as isize);
    if dir1Size > 0 as libc::c_int as libc::c_ulong
        && *buffer.offset(-(1 as libc::c_int as isize)) as libc::c_int != PATH_SEP
    {
        *buffer = PATH_SEP as libc::c_char;
        buffer = buffer.offset(1);
    }
    memcpy(buffer as *mut libc::c_void, dir2 as *const libc::c_void, dir2Size);
    *buffer.offset(dir2Size as isize) = '\0' as i32 as libc::c_char;
    return outDirBuffer;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_createMirroredDestDirName(
    mut srcFileName: *const libc::c_char,
    mut outDirRootName: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pathname = NULL as *mut libc::c_char;
    if isFileNameValidForMirroredOutput(srcFileName) == 0 {
        return NULL as *mut libc::c_char;
    }
    pathname = mallocAndJoin2Dir(outDirRootName, trimPath(srcFileName));
    convertPathnameToDirName(pathname);
    return pathname;
}
unsafe extern "C" fn mirrorSrcDir(
    mut srcDirName: *mut libc::c_char,
    mut outDirName: *const libc::c_char,
) -> libc::c_int {
    let mut srcMode: mode_t = 0;
    let mut status = 0 as libc::c_int;
    let mut newDir = mallocAndJoin2Dir(outDirName, trimPath(srcDirName));
    if newDir.is_null() {
        return -ENOMEM;
    }
    srcMode = getDirMode(srcDirName);
    status = makeDir(newDir, srcMode);
    free(newDir as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn mirrorSrcDirRecursive(
    mut srcDirName: *mut libc::c_char,
    mut outDirName: *const libc::c_char,
) -> libc::c_int {
    let mut status = 0 as libc::c_int;
    let mut pp = trimLeadingCurrentDir(srcDirName);
    let mut sp = NULL as *mut libc::c_char;
    loop {
        sp = strchr(pp, PATH_SEP);
        if sp.is_null() {
            break;
        }
        if sp != pp {
            *sp = '\0' as i32 as libc::c_char;
            status = mirrorSrcDir(srcDirName, outDirName);
            if status != 0 as libc::c_int {
                return status;
            }
            *sp = PATH_SEP as libc::c_char;
        }
        pp = sp.offset(1 as libc::c_int as isize);
    }
    status = mirrorSrcDir(srcDirName, outDirName);
    return status;
}
unsafe extern "C" fn makeMirroredDestDirsWithSameSrcDirMode(
    mut srcDirNames: *mut *mut libc::c_char,
    mut nbFile: libc::c_uint,
    mut outDirName: *const libc::c_char,
) {
    let mut i = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nbFile {
        mirrorSrcDirRecursive(*srcDirNames.offset(i as isize), outDirName);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn firstIsParentOrSameDirOfSecond(
    mut firstDir: *const libc::c_char,
    mut secondDir: *const libc::c_char,
) -> libc::c_int {
    let mut firstDirLen = strlen(firstDir);
    let mut secondDirLen = strlen(secondDir);
    return (firstDirLen <= secondDirLen
        && (*secondDir.offset(firstDirLen as isize) as libc::c_int == PATH_SEP
            || *secondDir.offset(firstDirLen as isize) as libc::c_int == '\0' as i32)
        && 0 as libc::c_int == strncmp(firstDir, secondDir, firstDirLen)) as libc::c_int;
}
unsafe extern "C" fn compareDir(
    mut pathname1: *const libc::c_void,
    mut pathname2: *const libc::c_void,
) -> libc::c_int {
    let mut s1 = trimPath(*(pathname1 as *const *mut libc::c_char));
    let mut s2 = trimPath(*(pathname2 as *const *mut libc::c_char));
    return strcmp(s1, s2);
}
unsafe extern "C" fn makeUniqueMirroredDestDirs(
    mut srcDirNames: *mut *mut libc::c_char,
    mut nbFile: libc::c_uint,
    mut outDirName: *const libc::c_char,
) {
    let mut i = 0 as libc::c_int as libc::c_uint;
    let mut uniqueDirNr = 0 as libc::c_int as libc::c_uint;
    let mut uniqueDirNames = NULL as *mut *mut libc::c_char;
    if nbFile == 0 as libc::c_int as libc::c_uint {
        return;
    }
    uniqueDirNames = malloc(
        (nbFile as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if uniqueDirNames.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                1221 as libc::c_int,
                b"uniqueDirNames != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    qsort(
        srcDirNames as *mut libc::c_void,
        nbFile as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            compareDir
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    uniqueDirNr = 1 as libc::c_int as libc::c_uint;
    let ref mut fresh4 = *uniqueDirNames
        .offset(uniqueDirNr.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *fresh4 = *srcDirNames.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int as libc::c_uint;
    while i < nbFile {
        let mut prevDirName = *srcDirNames
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        let mut currDirName = *srcDirNames.offset(i as isize);
        if firstIsParentOrSameDirOfSecond(trimPath(prevDirName), trimPath(currDirName))
            == 0
        {
            uniqueDirNr = uniqueDirNr.wrapping_add(1);
        }
        let ref mut fresh5 = *uniqueDirNames
            .offset(uniqueDirNr.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        *fresh5 = currDirName;
        i = i.wrapping_add(1);
    }
    makeMirroredDestDirsWithSameSrcDirMode(uniqueDirNames, uniqueDirNr, outDirName);
    free(uniqueDirNames as *mut libc::c_void);
}
unsafe extern "C" fn makeMirroredDestDirs(
    mut srcFileNames: *mut *mut libc::c_char,
    mut nbFile: libc::c_uint,
    mut outDirName: *const libc::c_char,
) {
    let mut i = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nbFile {
        convertPathnameToDirName(*srcFileNames.offset(i as isize));
        i = i.wrapping_add(1);
    }
    makeUniqueMirroredDestDirs(srcFileNames, nbFile, outDirName);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_mirrorSourceFilesDirectories(
    mut inFileNames: *mut *const libc::c_char,
    mut nbFile: libc::c_uint,
    mut outDirName: *const libc::c_char,
) {
    let mut i = 0 as libc::c_int as libc::c_uint;
    let mut validFilenamesNr = 0 as libc::c_int as libc::c_uint;
    let mut srcFileNames = malloc(
        (nbFile as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if srcFileNames.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                1263 as libc::c_int,
                b"srcFileNames != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < nbFile {
        if isFileNameValidForMirroredOutput(*inFileNames.offset(i as isize)) != 0 {
            let mut fname = strdup(*inFileNames.offset(i as isize));
            if fname.is_null() {
                if g_utilDisplayLevel >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                            as *const libc::c_char,
                        1269 as libc::c_int,
                        b"fname != NULL\0" as *const u8 as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
            let fresh6 = validFilenamesNr;
            validFilenamesNr = validFilenamesNr.wrapping_add(1);
            let ref mut fresh7 = *srcFileNames.offset(fresh6 as isize);
            *fresh7 = fname;
        }
        i = i.wrapping_add(1);
    }
    if validFilenamesNr > 0 as libc::c_int as libc::c_uint {
        makeDir(outDirName, DIR_DEFAULT_MODE as mode_t);
        makeMirroredDestDirs(srcFileNames, validFilenamesNr, outDirName);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < validFilenamesNr {
        free(*srcFileNames.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free(srcFileNames as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_createExpandedFNT(
    mut inputNames: *const *const libc::c_char,
    mut nbIfns: size_t,
    mut followLinks: libc::c_int,
) -> *mut FileNamesTable {
    let mut nbFiles: libc::c_uint = 0;
    let mut buf = malloc(LIST_SIZE_INCREASE as libc::c_ulong) as *mut libc::c_char;
    let mut bufend = buf.offset(LIST_SIZE_INCREASE as isize);
    if buf.is_null() {
        return NULL as *mut FileNamesTable;
    }
    let mut ifnNb: size_t = 0;
    let mut pos: size_t = 0;
    ifnNb = 0 as libc::c_int as size_t;
    pos = 0 as libc::c_int as size_t;
    nbFiles = 0 as libc::c_int as libc::c_uint;
    while ifnNb < nbIfns {
        if UTIL_isDirectory(*inputNames.offset(ifnNb as isize)) == 0 {
            let len = strlen(*inputNames.offset(ifnNb as isize));
            if buf.offset(pos as isize).offset(len as isize) >= bufend {
                let mut newListSize = bufend.offset_from(buf) as libc::c_long
                    + LIST_SIZE_INCREASE as libc::c_long;
                if newListSize >= 0 as libc::c_int as libc::c_long {} else {
                    __assert_fail(
                        b"newListSize >= 0\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                            as *const libc::c_char,
                        1299 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 73],
                            &[libc::c_char; 73],
                        >(
                            b"FileNamesTable *UTIL_createExpandedFNT(const char *const *, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                buf = UTIL_realloc(buf as *mut libc::c_void, newListSize as size_t)
                    as *mut libc::c_char;
                if buf.is_null() {
                    return NULL as *mut FileNamesTable;
                }
                bufend = buf.offset(newListSize as isize);
            }
            if buf.offset(pos as isize).offset(len as isize) < bufend {
                memcpy(
                    buf.offset(pos as isize) as *mut libc::c_void,
                    *inputNames.offset(ifnNb as isize) as *const libc::c_void,
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                pos = (pos as libc::c_ulong)
                    .wrapping_add(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                nbFiles = nbFiles.wrapping_add(1);
            }
        } else {
            nbFiles = nbFiles
                .wrapping_add(
                    UTIL_prepareFileList(
                        *inputNames.offset(ifnNb as isize),
                        &mut buf,
                        &mut pos,
                        &mut bufend,
                        followLinks,
                    ) as libc::c_uint,
                );
            if buf.is_null() {
                return NULL as *mut FileNamesTable;
            }
        }
        ifnNb = ifnNb.wrapping_add(1);
    }
    let mut ifnNb_0: size_t = 0;
    let mut pos_0: size_t = 0;
    let fntCapacity = nbFiles.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
    let fileNamesTable = malloc(
        fntCapacity
            .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    if fileNamesTable.is_null() {
        free(buf as *mut libc::c_void);
        return NULL as *mut FileNamesTable;
    }
    ifnNb_0 = 0 as libc::c_int as size_t;
    pos_0 = 0 as libc::c_int as size_t;
    while ifnNb_0 < nbFiles as libc::c_ulong {
        let ref mut fresh8 = *fileNamesTable.offset(ifnNb_0 as isize);
        *fresh8 = buf.offset(pos_0 as isize);
        if buf.offset(pos_0 as isize) > bufend {
            free(buf as *mut libc::c_void);
            free(fileNamesTable as *mut libc::c_void);
            return NULL as *mut FileNamesTable;
        }
        pos_0 = (pos_0 as libc::c_ulong)
            .wrapping_add(
                (strlen(*fileNamesTable.offset(ifnNb_0 as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        ifnNb_0 = ifnNb_0.wrapping_add(1);
    }
    return UTIL_assembleFileNamesTable2(
        fileNamesTable,
        nbFiles as size_t,
        fntCapacity,
        buf,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_expandFNT(
    mut fnt: *mut *mut FileNamesTable,
    mut followLinks: libc::c_int,
) {
    let newFNT = UTIL_createExpandedFNT(
        (**fnt).fileNames,
        (**fnt).tableSize,
        followLinks,
    );
    if newFNT.is_null() {
        if g_utilDisplayLevel >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const libc::c_char,
                b"/home/peter/Dev/zstd-c2rust/programs/util.c\0" as *const u8
                    as *const libc::c_char,
                1334 as libc::c_int,
                b"newFNT != NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    UTIL_freeFileNamesTable(*fnt);
    *fnt = newFNT;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_createFNT_fromROTable(
    mut filenames: *mut *const libc::c_char,
    mut nbFilenames: size_t,
) -> *mut FileNamesTable {
    let sizeof_FNTable = nbFilenames
        .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong);
    let newFNTable = malloc(sizeof_FNTable) as *mut *const libc::c_char;
    if newFNTable.is_null() {
        return NULL as *mut FileNamesTable;
    }
    memcpy(
        newFNTable as *mut libc::c_void,
        filenames as *const libc::c_void,
        sizeof_FNTable,
    );
    return UTIL_assembleFileNamesTable(
        newFNTable,
        nbFilenames,
        NULL as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_countCores(mut logical: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    static mut numCores: libc::c_int = 0 as libc::c_int;
    if numCores != 0 as libc::c_int {
        return numCores;
    }
    numCores = sysconf(_SC_NPROCESSORS_ONLN_0) as libc::c_int;
    if numCores == -(1 as libc::c_int) {
        numCores = 1 as libc::c_int;
        return numCores;
    }
    let cpuinfo = fopen(
        b"/proc/cpuinfo\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut siblings = 0 as libc::c_int;
    let mut cpu_cores = 0 as libc::c_int;
    let mut ratio = 1 as libc::c_int;
    if cpuinfo.is_null() {
        return numCores;
    }
    loop {
        if !(feof(cpuinfo) == 0) {
            current_block = 11584701595673473500;
            break;
        }
        if !(fgets(buff.as_mut_ptr(), BUF_SIZE, cpuinfo)).is_null() {
            if strncmp(
                buff.as_mut_ptr(),
                b"siblings\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                let sep: *const libc::c_char = strchr(buff.as_mut_ptr(), ':' as i32);
                if sep.is_null() || *sep as libc::c_int == '\0' as i32 {
                    current_block = 16443617707930808077;
                    break;
                } else {
                    siblings = atoi(sep.offset(1 as libc::c_int as isize));
                }
            }
            if !(strncmp(
                buff.as_mut_ptr(),
                b"cpu cores\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                continue;
            }
            let sep_0: *const libc::c_char = strchr(buff.as_mut_ptr(), ':' as i32);
            if sep_0.is_null() || *sep_0 as libc::c_int == '\0' as i32 {
                current_block = 16443617707930808077;
                break;
            } else {
                cpu_cores = atoi(sep_0.offset(1 as libc::c_int as isize));
            }
        } else if ferror(cpuinfo) != 0 {
            current_block = 16443617707930808077;
            break;
        }
    }
    match current_block {
        11584701595673473500 => {
            if siblings != 0 && cpu_cores != 0 && siblings > cpu_cores {
                ratio = siblings / cpu_cores;
            }
            if ratio != 0 && numCores > ratio && logical == 0 {
                numCores = numCores / ratio;
            }
        }
        _ => {}
    }
    fclose(cpuinfo);
    return numCores;
}
pub const BUF_SIZE: libc::c_int = 80 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn UTIL_countPhysicalCores() -> libc::c_int {
    return UTIL_countCores(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_countLogicalCores() -> libc::c_int {
    return UTIL_countCores(1 as libc::c_int);
}
