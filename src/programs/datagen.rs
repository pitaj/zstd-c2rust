use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type fixedPoint_24_8 = U32;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const LTLOG: libc::c_int = 13 as libc::c_int;
pub const LTSIZE: libc::c_int = (1 as libc::c_int) << LTLOG;
pub const LTMASK: libc::c_int = LTSIZE - 1 as libc::c_int;
unsafe extern "C" fn RDG_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = 2654435761 as libc::c_uint;
    static mut prime2: U32 = 2246822519 as libc::c_uint;
    let mut rand32 = *src;
    rand32 = (rand32 as libc::c_uint).wrapping_mul(prime1) as U32 as U32;
    rand32 ^= prime2;
    rand32 = rand32 << 13 as libc::c_int
        | rand32 >> 32 as libc::c_int - 13 as libc::c_int;
    *src = rand32;
    return rand32 >> 5 as libc::c_int;
}
unsafe extern "C" fn RDG_fillLiteralDistrib(
    mut ldt: *mut BYTE,
    mut ld: fixedPoint_24_8,
) {
    let firstChar = (if ld as libc::c_double <= 0.0f64 {
        0 as libc::c_int
    } else {
        '(' as i32
    }) as BYTE;
    let lastChar = (if ld as libc::c_double <= 0.0f64 {
        255 as libc::c_int
    } else {
        '}' as i32
    }) as BYTE;
    let mut character = (if ld as libc::c_double <= 0.0f64 {
        0 as libc::c_int
    } else {
        '0' as i32
    }) as BYTE;
    let mut u: U32 = 0;
    if ld <= 0 as libc::c_int as libc::c_uint {
        ld = 0 as libc::c_int as fixedPoint_24_8;
    }
    u = 0 as libc::c_int as U32;
    while u < LTSIZE as libc::c_uint {
        let weight = ((LTSIZE as libc::c_uint).wrapping_sub(u).wrapping_mul(ld)
            >> 8 as libc::c_int)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        let end = if u.wrapping_add(weight)
            < ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint
        {
            u.wrapping_add(weight)
        } else {
            ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint
        };
        while u < end {
            let fresh0 = u;
            u = u.wrapping_add(1);
            *ldt.offset(fresh0 as isize) = character;
        }
        character = character.wrapping_add(1);
        if character as libc::c_int > lastChar as libc::c_int {
            character = firstChar;
        }
    }
}
unsafe extern "C" fn RDG_genChar(mut seed: *mut U32, mut ldt: *const BYTE) -> BYTE {
    let id = RDG_rand(seed) & LTMASK as libc::c_uint;
    return *ldt.offset(id as isize);
}
unsafe extern "C" fn RDG_rand15Bits(mut seedPtr: *mut U32) -> U32 {
    return RDG_rand(seedPtr) & 0x7fff as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn RDG_randLength(mut seedPtr: *mut U32) -> U32 {
    if RDG_rand(seedPtr) & 7 as libc::c_int as libc::c_uint != 0 {
        return RDG_rand(seedPtr) & 0xf as libc::c_int as libc::c_uint;
    }
    return (RDG_rand(seedPtr) & 0x1ff as libc::c_int as libc::c_uint)
        .wrapping_add(0xf as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn RDG_genBlock(
    mut buffer: *mut libc::c_void,
    mut buffSize: size_t,
    mut prefixSize: size_t,
    mut matchProba: libc::c_double,
    mut ldt: *const BYTE,
    mut seedPtr: *mut U32,
) {
    let buffPtr = buffer as *mut BYTE;
    let matchProba32 = (32768 as libc::c_int as libc::c_double * matchProba) as U32;
    let mut pos = prefixSize;
    let mut prevOffset = 1 as libc::c_int as U32;
    while matchProba >= 1.0f64 {
        let mut size0 = (RDG_rand(seedPtr) & 3 as libc::c_int as libc::c_uint) as size_t;
        size0 = (1 as libc::c_int as size_t)
            << (16 as libc::c_int as libc::c_ulong)
                .wrapping_add(size0.wrapping_mul(2 as libc::c_int as libc::c_ulong));
        size0 = (size0 as libc::c_ulong)
            .wrapping_add(
                RDG_rand(seedPtr) as libc::c_ulong
                    & size0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        if buffSize < pos.wrapping_add(size0) {
            memset(
                buffPtr.offset(pos as isize) as *mut libc::c_void,
                0 as libc::c_int,
                buffSize.wrapping_sub(pos),
            );
            return;
        }
        memset(
            buffPtr.offset(pos as isize) as *mut libc::c_void,
            0 as libc::c_int,
            size0,
        );
        pos = (pos as libc::c_ulong).wrapping_add(size0) as size_t as size_t;
        *buffPtr
            .offset(
                pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = RDG_genChar(seedPtr, ldt);
    }
    if pos == 0 as libc::c_int as libc::c_ulong {
        *buffPtr.offset(0 as libc::c_int as isize) = RDG_genChar(seedPtr, ldt);
        pos = 1 as libc::c_int as size_t;
    }
    while pos < buffSize {
        if RDG_rand15Bits(seedPtr) < matchProba32 {
            let length = (RDG_randLength(seedPtr))
                .wrapping_add(4 as libc::c_int as libc::c_uint);
            let d = (if pos.wrapping_add(length as libc::c_ulong) < buffSize {
                pos.wrapping_add(length as libc::c_ulong)
            } else {
                buffSize
            }) as U32;
            let repeatOffset = (RDG_rand(seedPtr) & 15 as libc::c_int as libc::c_uint
                == 2 as libc::c_int as libc::c_uint) as libc::c_int as U32;
            let randOffset = (RDG_rand15Bits(seedPtr))
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            let offset = if repeatOffset != 0 {
                prevOffset
            } else {
                (if (randOffset as libc::c_ulong) < pos {
                    randOffset as libc::c_ulong
                } else {
                    pos
                }) as U32
            };
            let mut match_0 = pos.wrapping_sub(offset as libc::c_ulong);
            while pos < d as libc::c_ulong {
                let fresh1 = match_0;
                match_0 = match_0.wrapping_add(1);
                let fresh2 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh2 as isize) = *buffPtr.offset(fresh1 as isize);
            }
            prevOffset = offset;
        } else {
            let length_0 = RDG_randLength(seedPtr);
            let d_0 = (if pos.wrapping_add(length_0 as libc::c_ulong) < buffSize {
                pos.wrapping_add(length_0 as libc::c_ulong)
            } else {
                buffSize
            }) as U32;
            while pos < d_0 as libc::c_ulong {
                let fresh3 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh3 as isize) = RDG_genChar(seedPtr, ldt);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBuffer(
    mut buffer: *mut libc::c_void,
    mut size: size_t,
    mut matchProba: libc::c_double,
    mut litProba: libc::c_double,
    mut seed: libc::c_uint,
) {
    let mut seed32 = seed;
    let mut ldt: [BYTE; 8192] = [0; 8192];
    memset(
        ldt.as_mut_ptr() as *mut libc::c_void,
        '0' as i32,
        ::core::mem::size_of::<[BYTE; 8192]>() as libc::c_ulong,
    );
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256 as libc::c_int as libc::c_double + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(
        buffer,
        size,
        0 as libc::c_int as size_t,
        matchProba,
        ldt.as_mut_ptr(),
        &mut seed32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genStdout(
    mut size: libc::c_ulonglong,
    mut matchProba: libc::c_double,
    mut litProba: libc::c_double,
    mut seed: libc::c_uint,
) {
    let mut seed32 = seed;
    let stdBlockSize = (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
        as size_t;
    let stdDictSize = (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
        as size_t;
    let buff = malloc(stdDictSize.wrapping_add(stdBlockSize)) as *mut BYTE;
    let mut total = 0 as libc::c_int as U64;
    let mut ldt: [BYTE; 8192] = [0; 8192];
    if buff.is_null() {
        perror(b"datagen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    memset(
        ldt.as_mut_ptr() as *mut libc::c_void,
        '0' as i32,
        ::core::mem::size_of::<[BYTE; 8192]>() as libc::c_ulong,
    );
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256 as libc::c_int as libc::c_double + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(
        buff as *mut libc::c_void,
        stdDictSize,
        0 as libc::c_int as size_t,
        matchProba,
        ldt.as_mut_ptr(),
        &mut seed32,
    );
    while (total as libc::c_ulonglong) < size {
        let genBlockSize = (if (stdBlockSize as libc::c_ulonglong)
            < size.wrapping_sub(total as libc::c_ulonglong)
        {
            stdBlockSize as libc::c_ulonglong
        } else {
            size.wrapping_sub(total as libc::c_ulonglong)
        }) as size_t;
        RDG_genBlock(
            buff as *mut libc::c_void,
            stdDictSize.wrapping_add(stdBlockSize),
            stdDictSize,
            matchProba,
            ldt.as_mut_ptr(),
            &mut seed32,
        );
        total = (total as libc::c_ulong).wrapping_add(genBlockSize) as U64 as U64;
        let unused = fwrite(
            buff as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            genBlockSize,
            stdout,
        );
        memcpy(
            buff as *mut libc::c_void,
            buff.offset(stdBlockSize as isize) as *const libc::c_void,
            stdDictSize,
        );
    }
    free(buff as *mut libc::c_void);
}
