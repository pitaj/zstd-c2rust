use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type trbudget_t = _trbudget_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trbudget_t {
    pub chance: libc::c_int,
    pub remain: libc::c_int,
    pub incval: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub a: *const libc::c_int,
    pub b: *mut libc::c_int,
    pub c: *mut libc::c_int,
    pub d: libc::c_int,
    pub e: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: *mut libc::c_int,
    pub b: *mut libc::c_int,
    pub c: libc::c_int,
    pub d: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: *mut libc::c_int,
    pub b: *mut libc::c_int,
    pub c: *mut libc::c_int,
    pub d: libc::c_int,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ALPHABET_SIZE: libc::c_int = 256 as libc::c_int;
pub const BUCKET_A_SIZE: libc::c_int = 256 as libc::c_int;
pub const BUCKET_B_SIZE: libc::c_int = ALPHABET_SIZE * ALPHABET_SIZE;
pub const SS_INSERTIONSORT_THRESHOLD: libc::c_int = 8 as libc::c_int;
pub const SS_BLOCKSIZE: libc::c_int = 1024 as libc::c_int;
pub const TR_INSERTIONSORT_THRESHOLD: libc::c_int = 8 as libc::c_int;
static mut lg_table: [libc::c_int; 256] = [
    -(1 as libc::c_int),
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
];
#[inline]
unsafe extern "C" fn ss_ilg(mut n: libc::c_int) -> libc::c_int {
    return if n & 0xff00 as libc::c_int != 0 {
        8 as libc::c_int
            + lg_table[(n >> 8 as libc::c_int & 0xff as libc::c_int) as usize]
    } else {
        0 as libc::c_int
            + lg_table[(n >> 0 as libc::c_int & 0xff as libc::c_int) as usize]
    };
}
static mut sqq_table: [libc::c_int; 256] = [
    0 as libc::c_int,
    16 as libc::c_int,
    22 as libc::c_int,
    27 as libc::c_int,
    32 as libc::c_int,
    35 as libc::c_int,
    39 as libc::c_int,
    42 as libc::c_int,
    45 as libc::c_int,
    48 as libc::c_int,
    50 as libc::c_int,
    53 as libc::c_int,
    55 as libc::c_int,
    57 as libc::c_int,
    59 as libc::c_int,
    61 as libc::c_int,
    64 as libc::c_int,
    65 as libc::c_int,
    67 as libc::c_int,
    69 as libc::c_int,
    71 as libc::c_int,
    73 as libc::c_int,
    75 as libc::c_int,
    76 as libc::c_int,
    78 as libc::c_int,
    80 as libc::c_int,
    81 as libc::c_int,
    83 as libc::c_int,
    84 as libc::c_int,
    86 as libc::c_int,
    87 as libc::c_int,
    89 as libc::c_int,
    90 as libc::c_int,
    91 as libc::c_int,
    93 as libc::c_int,
    94 as libc::c_int,
    96 as libc::c_int,
    97 as libc::c_int,
    98 as libc::c_int,
    99 as libc::c_int,
    101 as libc::c_int,
    102 as libc::c_int,
    103 as libc::c_int,
    104 as libc::c_int,
    106 as libc::c_int,
    107 as libc::c_int,
    108 as libc::c_int,
    109 as libc::c_int,
    110 as libc::c_int,
    112 as libc::c_int,
    113 as libc::c_int,
    114 as libc::c_int,
    115 as libc::c_int,
    116 as libc::c_int,
    117 as libc::c_int,
    118 as libc::c_int,
    119 as libc::c_int,
    120 as libc::c_int,
    121 as libc::c_int,
    122 as libc::c_int,
    123 as libc::c_int,
    124 as libc::c_int,
    125 as libc::c_int,
    126 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    129 as libc::c_int,
    130 as libc::c_int,
    131 as libc::c_int,
    132 as libc::c_int,
    133 as libc::c_int,
    134 as libc::c_int,
    135 as libc::c_int,
    136 as libc::c_int,
    137 as libc::c_int,
    138 as libc::c_int,
    139 as libc::c_int,
    140 as libc::c_int,
    141 as libc::c_int,
    142 as libc::c_int,
    143 as libc::c_int,
    144 as libc::c_int,
    144 as libc::c_int,
    145 as libc::c_int,
    146 as libc::c_int,
    147 as libc::c_int,
    148 as libc::c_int,
    149 as libc::c_int,
    150 as libc::c_int,
    150 as libc::c_int,
    151 as libc::c_int,
    152 as libc::c_int,
    153 as libc::c_int,
    154 as libc::c_int,
    155 as libc::c_int,
    155 as libc::c_int,
    156 as libc::c_int,
    157 as libc::c_int,
    158 as libc::c_int,
    159 as libc::c_int,
    160 as libc::c_int,
    160 as libc::c_int,
    161 as libc::c_int,
    162 as libc::c_int,
    163 as libc::c_int,
    163 as libc::c_int,
    164 as libc::c_int,
    165 as libc::c_int,
    166 as libc::c_int,
    167 as libc::c_int,
    167 as libc::c_int,
    168 as libc::c_int,
    169 as libc::c_int,
    170 as libc::c_int,
    170 as libc::c_int,
    171 as libc::c_int,
    172 as libc::c_int,
    173 as libc::c_int,
    173 as libc::c_int,
    174 as libc::c_int,
    175 as libc::c_int,
    176 as libc::c_int,
    176 as libc::c_int,
    177 as libc::c_int,
    178 as libc::c_int,
    178 as libc::c_int,
    179 as libc::c_int,
    180 as libc::c_int,
    181 as libc::c_int,
    181 as libc::c_int,
    182 as libc::c_int,
    183 as libc::c_int,
    183 as libc::c_int,
    184 as libc::c_int,
    185 as libc::c_int,
    185 as libc::c_int,
    186 as libc::c_int,
    187 as libc::c_int,
    187 as libc::c_int,
    188 as libc::c_int,
    189 as libc::c_int,
    189 as libc::c_int,
    190 as libc::c_int,
    191 as libc::c_int,
    192 as libc::c_int,
    192 as libc::c_int,
    193 as libc::c_int,
    193 as libc::c_int,
    194 as libc::c_int,
    195 as libc::c_int,
    195 as libc::c_int,
    196 as libc::c_int,
    197 as libc::c_int,
    197 as libc::c_int,
    198 as libc::c_int,
    199 as libc::c_int,
    199 as libc::c_int,
    200 as libc::c_int,
    201 as libc::c_int,
    201 as libc::c_int,
    202 as libc::c_int,
    203 as libc::c_int,
    203 as libc::c_int,
    204 as libc::c_int,
    204 as libc::c_int,
    205 as libc::c_int,
    206 as libc::c_int,
    206 as libc::c_int,
    207 as libc::c_int,
    208 as libc::c_int,
    208 as libc::c_int,
    209 as libc::c_int,
    209 as libc::c_int,
    210 as libc::c_int,
    211 as libc::c_int,
    211 as libc::c_int,
    212 as libc::c_int,
    212 as libc::c_int,
    213 as libc::c_int,
    214 as libc::c_int,
    214 as libc::c_int,
    215 as libc::c_int,
    215 as libc::c_int,
    216 as libc::c_int,
    217 as libc::c_int,
    217 as libc::c_int,
    218 as libc::c_int,
    218 as libc::c_int,
    219 as libc::c_int,
    219 as libc::c_int,
    220 as libc::c_int,
    221 as libc::c_int,
    221 as libc::c_int,
    222 as libc::c_int,
    222 as libc::c_int,
    223 as libc::c_int,
    224 as libc::c_int,
    224 as libc::c_int,
    225 as libc::c_int,
    225 as libc::c_int,
    226 as libc::c_int,
    226 as libc::c_int,
    227 as libc::c_int,
    227 as libc::c_int,
    228 as libc::c_int,
    229 as libc::c_int,
    229 as libc::c_int,
    230 as libc::c_int,
    230 as libc::c_int,
    231 as libc::c_int,
    231 as libc::c_int,
    232 as libc::c_int,
    232 as libc::c_int,
    233 as libc::c_int,
    234 as libc::c_int,
    234 as libc::c_int,
    235 as libc::c_int,
    235 as libc::c_int,
    236 as libc::c_int,
    236 as libc::c_int,
    237 as libc::c_int,
    237 as libc::c_int,
    238 as libc::c_int,
    238 as libc::c_int,
    239 as libc::c_int,
    240 as libc::c_int,
    240 as libc::c_int,
    241 as libc::c_int,
    241 as libc::c_int,
    242 as libc::c_int,
    242 as libc::c_int,
    243 as libc::c_int,
    243 as libc::c_int,
    244 as libc::c_int,
    244 as libc::c_int,
    245 as libc::c_int,
    245 as libc::c_int,
    246 as libc::c_int,
    246 as libc::c_int,
    247 as libc::c_int,
    247 as libc::c_int,
    248 as libc::c_int,
    248 as libc::c_int,
    249 as libc::c_int,
    249 as libc::c_int,
    250 as libc::c_int,
    250 as libc::c_int,
    251 as libc::c_int,
    251 as libc::c_int,
    252 as libc::c_int,
    252 as libc::c_int,
    253 as libc::c_int,
    253 as libc::c_int,
    254 as libc::c_int,
    254 as libc::c_int,
    255 as libc::c_int,
];
#[inline]
unsafe extern "C" fn ss_isqrt(mut x: libc::c_int) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    if x >= SS_BLOCKSIZE * SS_BLOCKSIZE {
        return SS_BLOCKSIZE;
    }
    e = if x as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        if x as libc::c_uint & 0xff000000 as libc::c_uint != 0 {
            24 as libc::c_int
                + lg_table[(x >> 24 as libc::c_int & 0xff as libc::c_int) as usize]
        } else {
            16 as libc::c_int
                + lg_table[(x >> 16 as libc::c_int & 0xff as libc::c_int) as usize]
        }
    } else if x & 0xff00 as libc::c_int != 0 {
        8 as libc::c_int
            + lg_table[(x >> 8 as libc::c_int & 0xff as libc::c_int) as usize]
    } else {
        0 as libc::c_int
            + lg_table[(x >> 0 as libc::c_int & 0xff as libc::c_int) as usize]
    };
    if e >= 16 as libc::c_int {
        y = sqq_table[(x >> e - 6 as libc::c_int - (e & 1 as libc::c_int)) as usize]
            << (e >> 1 as libc::c_int) - 7 as libc::c_int;
        if e >= 24 as libc::c_int {
            y = y + 1 as libc::c_int + x / y >> 1 as libc::c_int;
        }
        y = y + 1 as libc::c_int + x / y >> 1 as libc::c_int;
    } else if e >= 8 as libc::c_int {
        y = (sqq_table[(x >> e - 6 as libc::c_int - (e & 1 as libc::c_int)) as usize]
            >> 7 as libc::c_int - (e >> 1 as libc::c_int)) + 1 as libc::c_int;
    } else {
        return sqq_table[x as usize] >> 4 as libc::c_int
    }
    return if x < y * y { y - 1 as libc::c_int } else { y };
}
#[inline]
unsafe extern "C" fn ss_compare(
    mut T: *const libc::c_uchar,
    mut p1: *const libc::c_int,
    mut p2: *const libc::c_int,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut U1 = 0 as *const libc::c_uchar;
    let mut U2 = 0 as *const libc::c_uchar;
    let mut U1n = 0 as *const libc::c_uchar;
    let mut U2n = 0 as *const libc::c_uchar;
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T
        .offset(*p1.offset(1 as libc::c_int as isize) as isize)
        .offset(2 as libc::c_int as isize);
    U2n = T
        .offset(*p2.offset(1 as libc::c_int as isize) as isize)
        .offset(2 as libc::c_int as isize);
    while U1 < U1n && U2 < U2n && *U1 as libc::c_int == *U2 as libc::c_int {
        U1 = U1.offset(1);
        U2 = U2.offset(1);
    }
    return if U1 < U1n {
        if U2 < U2n { *U1 as libc::c_int - *U2 as libc::c_int } else { 1 as libc::c_int }
    } else if U2 < U2n {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn ss_insertionsort(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut i = 0 as *mut libc::c_int;
    let mut j = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = last.offset(-(2 as libc::c_int as isize));
    while first <= i {
        t = *i;
        j = i.offset(1 as libc::c_int as isize);
        loop {
            r = ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize), depth);
            if !((0 as libc::c_int) < r) {
                break;
            }
            loop {
                *j.offset(-(1 as libc::c_int as isize)) = *j;
                j = j.offset(1);
                if !(j < last && *j < 0 as libc::c_int) {
                    break;
                }
            }
            if last <= j {
                break;
            }
        }
        if r == 0 as libc::c_int {
            *j = !*j;
        }
        *j.offset(-(1 as libc::c_int as isize)) = t;
        i = i.offset(-1);
    }
}
#[inline]
unsafe extern "C" fn ss_fixdown(
    mut Td: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut SA: *mut libc::c_int,
    mut i: libc::c_int,
    mut size: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    v = *SA.offset(i as isize);
    c = *Td.offset(*PA.offset(v as isize) as isize) as libc::c_int;
    loop {
        j = 2 as libc::c_int * i + 1 as libc::c_int;
        if !(j < size) {
            break;
        }
        let fresh0 = j;
        j = j + 1;
        k = fresh0;
        d = *Td.offset(*PA.offset(*SA.offset(k as isize) as isize) as isize)
            as libc::c_int;
        e = *Td.offset(*PA.offset(*SA.offset(j as isize) as isize) as isize)
            as libc::c_int;
        if d < e {
            k = j;
            d = e;
        }
        if d <= c {
            break;
        }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k;
    }
    *SA.offset(i as isize) = v;
}
unsafe extern "C" fn ss_heapsort(
    mut Td: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut SA: *mut libc::c_int,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    m = size;
    if size % 2 as libc::c_int == 0 as libc::c_int {
        m -= 1;
        if (*Td
            .offset(
                *PA.offset(*SA.offset((m / 2 as libc::c_int) as isize) as isize) as isize,
            ) as libc::c_int)
            < *Td.offset(*PA.offset(*SA.offset(m as isize) as isize) as isize)
                as libc::c_int
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2 as libc::c_int) as isize);
            *SA.offset((m / 2 as libc::c_int) as isize) = t;
        }
    }
    i = m / 2 as libc::c_int - 1 as libc::c_int;
    while 0 as libc::c_int <= i {
        ss_fixdown(Td, PA, SA, i, m);
        i -= 1;
    }
    if size % 2 as libc::c_int == 0 as libc::c_int {
        t = *SA.offset(0 as libc::c_int as isize);
        *SA.offset(0 as libc::c_int as isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        ss_fixdown(Td, PA, SA, 0 as libc::c_int, m);
    }
    i = m - 1 as libc::c_int;
    while (0 as libc::c_int) < i {
        t = *SA.offset(0 as libc::c_int as isize);
        *SA.offset(0 as libc::c_int as isize) = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0 as libc::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn ss_median3(
    mut Td: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut v1: *mut libc::c_int,
    mut v2: *mut libc::c_int,
    mut v3: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut t = 0 as *mut libc::c_int;
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int
    {
        t = v1;
        v1 = v2;
        v2 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int
    {
        if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int
            > *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int
        {
            return v1
        } else {
            return v3
        }
    }
    return v2;
}
#[inline]
unsafe extern "C" fn ss_median5(
    mut Td: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut v1: *mut libc::c_int,
    mut v2: *mut libc::c_int,
    mut v3: *mut libc::c_int,
    mut v4: *mut libc::c_int,
    mut v5: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut t = 0 as *mut libc::c_int;
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int
    {
        t = v2;
        v2 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v5 as isize) as isize) as libc::c_int
    {
        t = v4;
        v4 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int
    {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int
    {
        t = v1;
        v1 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int
    {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int
    {
        return v4;
    }
    return v3;
}
#[inline]
unsafe extern "C" fn ss_pivot(
    mut Td: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut middle = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    t = last.offset_from(first) as libc::c_long as libc::c_int;
    middle = first.offset((t / 2 as libc::c_int) as isize);
    if t <= 512 as libc::c_int {
        if t <= 32 as libc::c_int {
            return ss_median3(
                Td,
                PA,
                first,
                middle,
                last.offset(-(1 as libc::c_int as isize)),
            )
        } else {
            t >>= 2 as libc::c_int;
            return ss_median5(
                Td,
                PA,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as libc::c_int as isize)).offset(-(t as isize)),
                last.offset(-(1 as libc::c_int as isize)),
            );
        }
    }
    t >>= 3 as libc::c_int;
    first = ss_median3(
        Td,
        PA,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as libc::c_int) as isize),
    );
    middle = ss_median3(
        Td,
        PA,
        middle.offset(-(t as isize)),
        middle,
        middle.offset(t as isize),
    );
    last = ss_median3(
        Td,
        PA,
        last
            .offset(-(1 as libc::c_int as isize))
            .offset(-((t << 1 as libc::c_int) as isize)),
        last.offset(-(1 as libc::c_int as isize)).offset(-(t as isize)),
        last.offset(-(1 as libc::c_int as isize)),
    );
    return ss_median3(Td, PA, first, middle, last);
}
#[inline]
unsafe extern "C" fn ss_partition(
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) -> *mut libc::c_int {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    a = first.offset(-(1 as libc::c_int as isize));
    b = last;
    loop {
        loop {
            a = a.offset(1);
            if !(a < b
                && *PA.offset(*a as isize) + depth
                    >= *PA.offset((*a + 1 as libc::c_int) as isize) + 1 as libc::c_int)
            {
                break;
            }
            *a = !*a;
        }
        loop {
            b = b.offset(-1);
            if !(a < b
                && *PA.offset(*b as isize) + depth
                    < *PA.offset((*b + 1 as libc::c_int) as isize) + 1 as libc::c_int)
            {
                break;
            }
        }
        if b <= a {
            break;
        }
        t = !*b;
        *b = *a;
        *a = t;
    }
    if first < a {
        *first = !*first;
    }
    return a;
}
unsafe extern "C" fn ss_mintrosort(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut stack: [C2RustUnnamed_0; 16] = [C2RustUnnamed_0 {
        a: 0 as *mut libc::c_int,
        b: 0 as *mut libc::c_int,
        c: 0,
        d: 0,
    }; 16];
    let mut Td = 0 as *const libc::c_uchar;
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut c = 0 as *mut libc::c_int;
    let mut d = 0 as *mut libc::c_int;
    let mut e = 0 as *mut libc::c_int;
    let mut f = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x = 0 as libc::c_int;
    ssize = 0 as libc::c_int;
    limit = ss_ilg(last.offset_from(first) as libc::c_long as libc::c_int);
    loop {
        if last.offset_from(first) as libc::c_long
            <= SS_INSERTIONSORT_THRESHOLD as libc::c_long
        {
            if (1 as libc::c_int as libc::c_long)
                < last.offset_from(first) as libc::c_long
            {
                ss_insertionsort(T, PA, first, last, depth);
            }
            if 0 as libc::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    418 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 as libc::c_int {
                return;
            }
            ssize -= 1;
            first = stack[ssize as usize].a;
            last = stack[ssize as usize].b;
            depth = stack[ssize as usize].c;
            limit = stack[ssize as usize].d;
        } else {
            Td = T.offset(depth as isize);
            let fresh1 = limit;
            limit = limit - 1;
            if fresh1 == 0 as libc::c_int {
                ss_heapsort(
                    Td,
                    PA,
                    first,
                    last.offset_from(first) as libc::c_long as libc::c_int,
                );
            }
            if limit < 0 as libc::c_int {
                a = first.offset(1 as libc::c_int as isize);
                v = *Td.offset(*PA.offset(*first as isize) as isize) as libc::c_int;
                while a < last {
                    x = *Td.offset(*PA.offset(*a as isize) as isize) as libc::c_int;
                    if x != v {
                        if (1 as libc::c_int as libc::c_long)
                            < a.offset_from(first) as libc::c_long
                        {
                            break;
                        }
                        v = x;
                        first = a;
                    }
                    a = a.offset(1);
                }
                if (*Td.offset((*PA.offset(*first as isize) - 1 as libc::c_int) as isize)
                    as libc::c_int) < v
                {
                    first = ss_partition(PA, first, a, depth);
                }
                if a.offset_from(first) as libc::c_long
                    <= last.offset_from(a) as libc::c_long
                {
                    if (1 as libc::c_int as libc::c_long)
                        < a.offset_from(first) as libc::c_long
                    {
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                437 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = a;
                        stack[ssize as usize].b = last;
                        stack[ssize as usize].c = depth;
                        let fresh2 = ssize;
                        ssize = ssize + 1;
                        stack[fresh2 as usize].d = -(1 as libc::c_int);
                        last = a;
                        depth += 1 as libc::c_int;
                        limit = ss_ilg(
                            a.offset_from(first) as libc::c_long as libc::c_int,
                        );
                    } else {
                        first = a;
                        limit = -(1 as libc::c_int);
                    }
                } else if (1 as libc::c_int as libc::c_long)
                    < last.offset_from(a) as libc::c_long
                {
                    if ssize < 16 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            444 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 74],
                                &[libc::c_char; 74],
                            >(
                                b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = a;
                    stack[ssize as usize].c = depth + 1 as libc::c_int;
                    let fresh3 = ssize;
                    ssize = ssize + 1;
                    stack[fresh3 as usize]
                        .d = ss_ilg(a.offset_from(first) as libc::c_long as libc::c_int);
                    first = a;
                    limit = -(1 as libc::c_int);
                } else {
                    last = a;
                    depth += 1 as libc::c_int;
                    limit = ss_ilg(a.offset_from(first) as libc::c_long as libc::c_int);
                }
            } else {
                a = ss_pivot(Td, PA, first, last);
                v = *Td.offset(*PA.offset(*a as isize) as isize) as libc::c_int;
                t = *first;
                *first = *a;
                *a = t;
                b = first;
                loop {
                    b = b.offset(1);
                    if !(b < last
                        && {
                            x = *Td.offset(*PA.offset(*b as isize) as isize)
                                as libc::c_int;
                            x == v
                        })
                    {
                        break;
                    }
                }
                a = b;
                if a < last && x < v {
                    loop {
                        b = b.offset(1);
                        if !(b < last
                            && {
                                x = *Td.offset(*PA.offset(*b as isize) as isize)
                                    as libc::c_int;
                                x <= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                        }
                    }
                }
                c = last;
                loop {
                    c = c.offset(-1);
                    if !(b < c
                        && {
                            x = *Td.offset(*PA.offset(*c as isize) as isize)
                                as libc::c_int;
                            x == v
                        })
                    {
                        break;
                    }
                }
                d = c;
                if b < d && x > v {
                    loop {
                        c = c.offset(-1);
                        if !(b < c
                            && {
                                x = *Td.offset(*PA.offset(*c as isize) as isize)
                                    as libc::c_int;
                                x >= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                        }
                    }
                }
                while b < c {
                    t = *b;
                    *b = *c;
                    *c = t;
                    loop {
                        b = b.offset(1);
                        if !(b < c
                            && {
                                x = *Td.offset(*PA.offset(*b as isize) as isize)
                                    as libc::c_int;
                                x <= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                        }
                    }
                    loop {
                        c = c.offset(-1);
                        if !(b < c
                            && {
                                x = *Td.offset(*PA.offset(*c as isize) as isize)
                                    as libc::c_int;
                                x >= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                        }
                    }
                }
                if a <= d {
                    c = b.offset(-(1 as libc::c_int as isize));
                    s = a.offset_from(first) as libc::c_long as libc::c_int;
                    t = b.offset_from(a) as libc::c_long as libc::c_int;
                    if s > t {
                        s = t;
                    }
                    e = first;
                    f = b.offset(-(s as isize));
                    while (0 as libc::c_int) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1);
                        f = f.offset(1);
                    }
                    s = d.offset_from(c) as libc::c_long as libc::c_int;
                    t = (last.offset_from(d) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                    if s > t {
                        s = t;
                    }
                    e = b;
                    f = last.offset(-(s as isize));
                    while (0 as libc::c_int) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1);
                        f = f.offset(1);
                    }
                    a = first.offset(b.offset_from(a) as libc::c_long as isize);
                    c = last.offset(-(d.offset_from(c) as libc::c_long as isize));
                    b = if v
                        <= *Td
                            .offset(
                                (*PA.offset(*a as isize) - 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    {
                        a
                    } else {
                        ss_partition(PA, a, c, depth)
                    };
                    if a.offset_from(first) as libc::c_long
                        <= last.offset_from(c) as libc::c_long
                    {
                        if last.offset_from(c) as libc::c_long
                            <= c.offset_from(b) as libc::c_long
                        {
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    494 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1 as libc::c_int;
                            let fresh4 = ssize;
                            ssize = ssize + 1;
                            stack[fresh4 as usize]
                                .d = ss_ilg(
                                c.offset_from(b) as libc::c_long as libc::c_int,
                            );
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    495 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh5 = ssize;
                            ssize = ssize + 1;
                            stack[fresh5 as usize].d = limit;
                            last = a;
                        } else if a.offset_from(first) as libc::c_long
                            <= c.offset_from(b) as libc::c_long
                        {
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    498 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh6 = ssize;
                            ssize = ssize + 1;
                            stack[fresh6 as usize].d = limit;
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    499 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1 as libc::c_int;
                            let fresh7 = ssize;
                            ssize = ssize + 1;
                            stack[fresh7 as usize]
                                .d = ss_ilg(
                                c.offset_from(b) as libc::c_long as libc::c_int,
                            );
                            last = a;
                        } else {
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    502 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh8 = ssize;
                            ssize = ssize + 1;
                            stack[fresh8 as usize].d = limit;
                            if ssize < 16 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    503 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[libc::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = first;
                            stack[ssize as usize].b = a;
                            stack[ssize as usize].c = depth;
                            let fresh9 = ssize;
                            ssize = ssize + 1;
                            stack[fresh9 as usize].d = limit;
                            first = b;
                            last = c;
                            depth += 1 as libc::c_int;
                            limit = ss_ilg(
                                c.offset_from(b) as libc::c_long as libc::c_int,
                            );
                        }
                    } else if a.offset_from(first) as libc::c_long
                        <= c.offset_from(b) as libc::c_long
                    {
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                508 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1 as libc::c_int;
                        let fresh10 = ssize;
                        ssize = ssize + 1;
                        stack[fresh10 as usize]
                            .d = ss_ilg(c.offset_from(b) as libc::c_long as libc::c_int);
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                509 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh11 = ssize;
                        ssize = ssize + 1;
                        stack[fresh11 as usize].d = limit;
                        first = c;
                    } else if last.offset_from(c) as libc::c_long
                        <= c.offset_from(b) as libc::c_long
                    {
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                512 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh12 = ssize;
                        ssize = ssize + 1;
                        stack[fresh12 as usize].d = limit;
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                513 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1 as libc::c_int;
                        let fresh13 = ssize;
                        ssize = ssize + 1;
                        stack[fresh13 as usize]
                            .d = ss_ilg(c.offset_from(b) as libc::c_long as libc::c_int);
                        first = c;
                    } else {
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                516 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh14 = ssize;
                        ssize = ssize + 1;
                        stack[fresh14 as usize].d = limit;
                        if ssize < 16 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                517 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[libc::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = c;
                        stack[ssize as usize].b = last;
                        stack[ssize as usize].c = depth;
                        let fresh15 = ssize;
                        ssize = ssize + 1;
                        stack[fresh15 as usize].d = limit;
                        first = b;
                        last = c;
                        depth += 1 as libc::c_int;
                        limit = ss_ilg(c.offset_from(b) as libc::c_long as libc::c_int);
                    }
                } else {
                    limit += 1 as libc::c_int;
                    if (*Td
                        .offset(
                            (*PA.offset(*first as isize) - 1 as libc::c_int) as isize,
                        ) as libc::c_int) < v
                    {
                        first = ss_partition(PA, first, last, depth);
                        limit = ss_ilg(
                            last.offset_from(first) as libc::c_long as libc::c_int,
                        );
                    }
                    depth += 1 as libc::c_int;
                }
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ss_blockswap(
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
    mut n: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    while (0 as libc::c_int) < n {
        t = *a;
        *a = *b;
        *b = t;
        n -= 1;
        a = a.offset(1);
        b = b.offset(1);
    }
}
#[inline]
unsafe extern "C" fn ss_rotate(
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
) {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    l = middle.offset_from(first) as libc::c_long as libc::c_int;
    r = last.offset_from(middle) as libc::c_long as libc::c_int;
    while (0 as libc::c_int) < l && (0 as libc::c_int) < r {
        if l == r {
            ss_blockswap(first, middle, l);
            break;
        } else if l < r {
            a = last.offset(-(1 as libc::c_int as isize));
            b = middle.offset(-(1 as libc::c_int as isize));
            t = *a;
            loop {
                let fresh16 = a;
                a = a.offset(-1);
                *fresh16 = *b;
                let fresh17 = b;
                b = b.offset(-1);
                *fresh17 = *a;
                if !(b < first) {
                    continue;
                }
                *a = t;
                last = a;
                r -= l + 1 as libc::c_int;
                if r <= l {
                    break;
                }
                a = a.offset(-(1 as libc::c_int as isize));
                b = middle.offset(-(1 as libc::c_int as isize));
                t = *a;
            }
        } else {
            a = first;
            b = middle;
            t = *a;
            loop {
                let fresh18 = a;
                a = a.offset(1);
                *fresh18 = *b;
                let fresh19 = b;
                b = b.offset(1);
                *fresh19 = *a;
                if !(last <= b) {
                    continue;
                }
                *a = t;
                first = a.offset(1 as libc::c_int as isize);
                l -= r + 1 as libc::c_int;
                if l <= r {
                    break;
                }
                a = a.offset(1 as libc::c_int as isize);
                b = middle;
                t = *a;
            }
        }
    }
}
unsafe extern "C" fn ss_inplacemerge(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut p = 0 as *const libc::c_int;
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut len: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    loop {
        if *last.offset(-(1 as libc::c_int as isize)) < 0 as libc::c_int {
            x = 1 as libc::c_int;
            p = PA.offset(!*last.offset(-(1 as libc::c_int as isize)) as isize);
        } else {
            x = 0 as libc::c_int;
            p = PA.offset(*last.offset(-(1 as libc::c_int as isize)) as isize);
        }
        a = first;
        len = middle.offset_from(first) as libc::c_long as libc::c_int;
        half = len >> 1 as libc::c_int;
        r = -(1 as libc::c_int);
        while (0 as libc::c_int) < len {
            b = a.offset(half as isize);
            q = ss_compare(
                T,
                PA.offset((if 0 as libc::c_int <= *b { *b } else { !*b }) as isize),
                p,
                depth,
            );
            if q < 0 as libc::c_int {
                a = b.offset(1 as libc::c_int as isize);
                half -= len & 1 as libc::c_int ^ 1 as libc::c_int;
            } else {
                r = q;
            }
            len = half;
            half >>= 1 as libc::c_int;
        }
        if a < middle {
            if r == 0 as libc::c_int {
                *a = !*a;
            }
            ss_rotate(a, middle, last);
            last = last.offset(-(middle.offset_from(a) as libc::c_long as isize));
            middle = a;
            if first == middle {
                break;
            }
        }
        last = last.offset(-1);
        if x != 0 as libc::c_int {
            loop {
                last = last.offset(-1);
                if !(*last < 0 as libc::c_int) {
                    break;
                }
            }
        }
        if middle == last {
            break;
        }
    };
}
unsafe extern "C" fn ss_mergeforward(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut buf: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut c = 0 as *mut libc::c_int;
    let mut bufend = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    bufend = buf
        .offset(middle.offset_from(first) as libc::c_long as isize)
        .offset(-(1 as libc::c_int as isize));
    ss_blockswap(buf, first, middle.offset_from(first) as libc::c_long as libc::c_int);
    a = first;
    t = *a;
    b = buf;
    c = middle;
    loop {
        r = ss_compare(T, PA.offset(*b as isize), PA.offset(*c as isize), depth);
        if r < 0 as libc::c_int {
            loop {
                let fresh20 = a;
                a = a.offset(1);
                *fresh20 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh21 = b;
                b = b.offset(1);
                *fresh21 = *a;
                if !(*b < 0 as libc::c_int) {
                    break;
                }
            }
        } else if r > 0 as libc::c_int {
            loop {
                let fresh22 = a;
                a = a.offset(1);
                *fresh22 = *c;
                let fresh23 = c;
                c = c.offset(1);
                *fresh23 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh24 = a;
                        a = a.offset(1);
                        *fresh24 = *b;
                        let fresh25 = b;
                        b = b.offset(1);
                        *fresh25 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if !(*c < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            *c = !*c;
            loop {
                let fresh26 = a;
                a = a.offset(1);
                *fresh26 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh27 = b;
                b = b.offset(1);
                *fresh27 = *a;
                if !(*b < 0 as libc::c_int) {
                    break;
                }
            }
            loop {
                let fresh28 = a;
                a = a.offset(1);
                *fresh28 = *c;
                let fresh29 = c;
                c = c.offset(1);
                *fresh29 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh30 = a;
                        a = a.offset(1);
                        *fresh30 = *b;
                        let fresh31 = b;
                        b = b.offset(1);
                        *fresh31 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if !(*c < 0 as libc::c_int) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn ss_mergebackward(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut buf: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut p1 = 0 as *const libc::c_int;
    let mut p2 = 0 as *const libc::c_int;
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut c = 0 as *mut libc::c_int;
    let mut bufend = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    bufend = buf
        .offset(last.offset_from(middle) as libc::c_long as isize)
        .offset(-(1 as libc::c_int as isize));
    ss_blockswap(buf, middle, last.offset_from(middle) as libc::c_long as libc::c_int);
    x = 0 as libc::c_int;
    if *bufend < 0 as libc::c_int {
        p1 = PA.offset(!*bufend as isize);
        x |= 1 as libc::c_int;
    } else {
        p1 = PA.offset(*bufend as isize);
    }
    if *middle.offset(-(1 as libc::c_int as isize)) < 0 as libc::c_int {
        p2 = PA.offset(!*middle.offset(-(1 as libc::c_int as isize)) as isize);
        x |= 2 as libc::c_int;
    } else {
        p2 = PA.offset(*middle.offset(-(1 as libc::c_int as isize)) as isize);
    }
    a = last.offset(-(1 as libc::c_int as isize));
    t = *a;
    b = bufend;
    c = middle.offset(-(1 as libc::c_int as isize));
    loop {
        r = ss_compare(T, p1, p2, depth);
        if (0 as libc::c_int) < r {
            if x & 1 as libc::c_int != 0 {
                loop {
                    let fresh32 = a;
                    a = a.offset(-1);
                    *fresh32 = *b;
                    let fresh33 = b;
                    b = b.offset(-1);
                    *fresh33 = *a;
                    if !(*b < 0 as libc::c_int) {
                        break;
                    }
                }
                x ^= 1 as libc::c_int;
            }
            let fresh34 = a;
            a = a.offset(-1);
            *fresh34 = *b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh35 = b;
                b = b.offset(-1);
                *fresh35 = *a;
                if *b < 0 as libc::c_int {
                    p1 = PA.offset(!*b as isize);
                    x |= 1 as libc::c_int;
                } else {
                    p1 = PA.offset(*b as isize);
                }
            }
        } else if r < 0 as libc::c_int {
            if x & 2 as libc::c_int != 0 {
                loop {
                    let fresh36 = a;
                    a = a.offset(-1);
                    *fresh36 = *c;
                    let fresh37 = c;
                    c = c.offset(-1);
                    *fresh37 = *a;
                    if !(*c < 0 as libc::c_int) {
                        break;
                    }
                }
                x ^= 2 as libc::c_int;
            }
            let fresh38 = a;
            a = a.offset(-1);
            *fresh38 = *c;
            let fresh39 = c;
            c = c.offset(-1);
            *fresh39 = *a;
            if c < first {
                while buf < b {
                    let fresh40 = a;
                    a = a.offset(-1);
                    *fresh40 = *b;
                    let fresh41 = b;
                    b = b.offset(-1);
                    *fresh41 = *a;
                }
                *a = *b;
                *b = t;
                break;
            } else if *c < 0 as libc::c_int {
                p2 = PA.offset(!*c as isize);
                x |= 2 as libc::c_int;
            } else {
                p2 = PA.offset(*c as isize);
            }
        } else {
            if x & 1 as libc::c_int != 0 {
                loop {
                    let fresh42 = a;
                    a = a.offset(-1);
                    *fresh42 = *b;
                    let fresh43 = b;
                    b = b.offset(-1);
                    *fresh43 = *a;
                    if !(*b < 0 as libc::c_int) {
                        break;
                    }
                }
                x ^= 1 as libc::c_int;
            }
            let fresh44 = a;
            a = a.offset(-1);
            *fresh44 = !*b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh45 = b;
                b = b.offset(-1);
                *fresh45 = *a;
                if x & 2 as libc::c_int != 0 {
                    loop {
                        let fresh46 = a;
                        a = a.offset(-1);
                        *fresh46 = *c;
                        let fresh47 = c;
                        c = c.offset(-1);
                        *fresh47 = *a;
                        if !(*c < 0 as libc::c_int) {
                            break;
                        }
                    }
                    x ^= 2 as libc::c_int;
                }
                let fresh48 = a;
                a = a.offset(-1);
                *fresh48 = *c;
                let fresh49 = c;
                c = c.offset(-1);
                *fresh49 = *a;
                if c < first {
                    while buf < b {
                        let fresh50 = a;
                        a = a.offset(-1);
                        *fresh50 = *b;
                        let fresh51 = b;
                        b = b.offset(-1);
                        *fresh51 = *a;
                    }
                    *a = *b;
                    *b = t;
                    break;
                } else {
                    if *b < 0 as libc::c_int {
                        p1 = PA.offset(!*b as isize);
                        x |= 1 as libc::c_int;
                    } else {
                        p1 = PA.offset(*b as isize);
                    }
                    if *c < 0 as libc::c_int {
                        p2 = PA.offset(!*c as isize);
                        x |= 2 as libc::c_int;
                    } else {
                        p2 = PA.offset(*c as isize);
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn ss_swapmerge(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut buf: *mut libc::c_int,
    mut bufsize: libc::c_int,
    mut depth: libc::c_int,
) {
    let mut stack: [C2RustUnnamed_1; 32] = [C2RustUnnamed_1 {
        a: 0 as *mut libc::c_int,
        b: 0 as *mut libc::c_int,
        c: 0 as *mut libc::c_int,
        d: 0,
    }; 32];
    let mut l = 0 as *mut libc::c_int;
    let mut r = 0 as *mut libc::c_int;
    let mut lm = 0 as *mut libc::c_int;
    let mut rm = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut check: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    check = 0 as libc::c_int;
    ssize = 0 as libc::c_int;
    loop {
        if last.offset_from(middle) as libc::c_long <= bufsize as libc::c_long {
            if first < middle && middle < last {
                ss_mergebackward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 as libc::c_int != 0
                || check & 2 as libc::c_int != 0
                    && ss_compare(
                        T,
                        PA
                            .offset(
                                (if 0 as libc::c_int
                                    <= *first.offset(-(1 as libc::c_int as isize))
                                {
                                    *first.offset(-(1 as libc::c_int as isize))
                                } else {
                                    !*first.offset(-(1 as libc::c_int as isize))
                                }) as isize,
                            ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0 as libc::c_int
            {
                *first = !*first;
            }
            if check & 4 as libc::c_int != 0
                && ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as libc::c_int
                                <= *last.offset(-(1 as libc::c_int as isize))
                            {
                                *last.offset(-(1 as libc::c_int as isize))
                            } else {
                                !*last.offset(-(1 as libc::c_int as isize))
                            }) as isize,
                        ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0 as libc::c_int
            {
                *last = !*last;
            }
            if 0 as libc::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    771 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[libc::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 as libc::c_int {
                return;
            }
            ssize -= 1;
            first = stack[ssize as usize].a;
            middle = stack[ssize as usize].b;
            last = stack[ssize as usize].c;
            check = stack[ssize as usize].d;
        } else if middle.offset_from(first) as libc::c_long <= bufsize as libc::c_long {
            if first < middle {
                ss_mergeforward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 as libc::c_int != 0
                || check & 2 as libc::c_int != 0
                    && ss_compare(
                        T,
                        PA
                            .offset(
                                (if 0 as libc::c_int
                                    <= *first.offset(-(1 as libc::c_int as isize))
                                {
                                    *first.offset(-(1 as libc::c_int as isize))
                                } else {
                                    !*first.offset(-(1 as libc::c_int as isize))
                                }) as isize,
                            ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0 as libc::c_int
            {
                *first = !*first;
            }
            if check & 4 as libc::c_int != 0
                && ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as libc::c_int
                                <= *last.offset(-(1 as libc::c_int as isize))
                            {
                                *last.offset(-(1 as libc::c_int as isize))
                            } else {
                                !*last.offset(-(1 as libc::c_int as isize))
                            }) as isize,
                        ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0 as libc::c_int
            {
                *last = !*last;
            }
            if 0 as libc::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    780 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[libc::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 as libc::c_int {
                return;
            }
            ssize -= 1;
            first = stack[ssize as usize].a;
            middle = stack[ssize as usize].b;
            last = stack[ssize as usize].c;
            check = stack[ssize as usize].d;
        } else {
            m = 0 as libc::c_int;
            len = (if (middle.offset_from(first) as libc::c_long)
                < last.offset_from(middle) as libc::c_long
            {
                middle.offset_from(first) as libc::c_long
            } else {
                last.offset_from(middle) as libc::c_long
            }) as libc::c_int;
            half = len >> 1 as libc::c_int;
            while (0 as libc::c_int) < len {
                if ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as libc::c_int
                                <= *middle.offset(m as isize).offset(half as isize)
                            {
                                *middle.offset(m as isize).offset(half as isize)
                            } else {
                                !*middle.offset(m as isize).offset(half as isize)
                            }) as isize,
                        ),
                    PA
                        .offset(
                            (if 0 as libc::c_int
                                <= *middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as libc::c_int as isize))
                            {
                                *middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as libc::c_int as isize))
                            } else {
                                !*middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as libc::c_int as isize))
                            }) as isize,
                        ),
                    depth,
                ) < 0 as libc::c_int
                {
                    m += half + 1 as libc::c_int;
                    half -= len & 1 as libc::c_int ^ 1 as libc::c_int;
                }
                len = half;
                half >>= 1 as libc::c_int;
            }
            if (0 as libc::c_int) < m {
                lm = middle.offset(-(m as isize));
                rm = middle.offset(m as isize);
                ss_blockswap(lm, middle, m);
                r = middle;
                l = r;
                next = 0 as libc::c_int;
                if rm < last {
                    if *rm < 0 as libc::c_int {
                        *rm = !*rm;
                        if first < lm {
                            loop {
                                l = l.offset(-1);
                                if !(*l < 0 as libc::c_int) {
                                    break;
                                }
                            }
                            next |= 4 as libc::c_int;
                        }
                        next |= 1 as libc::c_int;
                    } else if first < lm {
                        while *r < 0 as libc::c_int {
                            r = r.offset(1);
                        }
                        next |= 2 as libc::c_int;
                    }
                }
                if l.offset_from(first) as libc::c_long
                    <= last.offset_from(r) as libc::c_long
                {
                    if ssize < 32 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            810 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[libc::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = r;
                    stack[ssize as usize].b = rm;
                    stack[ssize as usize].c = last;
                    let fresh52 = ssize;
                    ssize = ssize + 1;
                    stack[fresh52 as usize]
                        .d = next & 3 as libc::c_int | check & 4 as libc::c_int;
                    middle = lm;
                    last = l;
                    check = check & 3 as libc::c_int | next & 4 as libc::c_int;
                } else {
                    if next & 2 as libc::c_int != 0 && r == middle {
                        next ^= 6 as libc::c_int;
                    }
                    if ssize < 32 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            814 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[libc::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = lm;
                    stack[ssize as usize].c = l;
                    let fresh53 = ssize;
                    ssize = ssize + 1;
                    stack[fresh53 as usize]
                        .d = check & 3 as libc::c_int | next & 4 as libc::c_int;
                    first = r;
                    middle = rm;
                    check = next & 3 as libc::c_int | check & 4 as libc::c_int;
                }
            } else {
                if ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as libc::c_int
                                <= *middle.offset(-(1 as libc::c_int as isize))
                            {
                                *middle.offset(-(1 as libc::c_int as isize))
                            } else {
                                !*middle.offset(-(1 as libc::c_int as isize))
                            }) as isize,
                        ),
                    PA.offset(*middle as isize),
                    depth,
                ) == 0 as libc::c_int
                {
                    *middle = !*middle;
                }
                if check & 1 as libc::c_int != 0
                    || check & 2 as libc::c_int != 0
                        && ss_compare(
                            T,
                            PA
                                .offset(
                                    (if 0 as libc::c_int
                                        <= *first.offset(-(1 as libc::c_int as isize))
                                    {
                                        *first.offset(-(1 as libc::c_int as isize))
                                    } else {
                                        !*first.offset(-(1 as libc::c_int as isize))
                                    }) as isize,
                                ),
                            PA.offset(*first as isize),
                            depth,
                        ) == 0 as libc::c_int
                {
                    *first = !*first;
                }
                if check & 4 as libc::c_int != 0
                    && ss_compare(
                        T,
                        PA
                            .offset(
                                (if 0 as libc::c_int
                                    <= *last.offset(-(1 as libc::c_int as isize))
                                {
                                    *last.offset(-(1 as libc::c_int as isize))
                                } else {
                                    !*last.offset(-(1 as libc::c_int as isize))
                                }) as isize,
                            ),
                        PA.offset(*last as isize),
                        depth,
                    ) == 0 as libc::c_int
                {
                    *last = !*last;
                }
                if 0 as libc::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const libc::c_char,
                        822 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[libc::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if ssize == 0 as libc::c_int {
                    return;
                }
                ssize -= 1;
                first = stack[ssize as usize].a;
                middle = stack[ssize as usize].b;
                last = stack[ssize as usize].c;
                check = stack[ssize as usize].d;
            }
        }
    };
}
unsafe extern "C" fn sssort(
    mut T: *const libc::c_uchar,
    mut PA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut buf: *mut libc::c_int,
    mut bufsize: libc::c_int,
    mut depth: libc::c_int,
    mut n: libc::c_int,
    mut lastsuffix: libc::c_int,
) {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut middle = 0 as *mut libc::c_int;
    let mut curbuf = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut curbufsize: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if lastsuffix != 0 as libc::c_int {
        first = first.offset(1);
    }
    if bufsize < SS_BLOCKSIZE
        && (bufsize as libc::c_long) < last.offset_from(first) as libc::c_long
        && {
            limit = ss_isqrt(last.offset_from(first) as libc::c_long as libc::c_int);
            bufsize < limit
        }
    {
        if SS_BLOCKSIZE < limit {
            limit = SS_BLOCKSIZE;
        }
        middle = last.offset(-(limit as isize));
        buf = middle;
        bufsize = limit;
    } else {
        middle = last;
        limit = 0 as libc::c_int;
    }
    a = first;
    i = 0 as libc::c_int;
    while (SS_BLOCKSIZE as libc::c_long) < middle.offset_from(a) as libc::c_long {
        ss_mintrosort(T, PA, a, a.offset(SS_BLOCKSIZE as isize), depth);
        curbufsize = last.offset_from(a.offset(SS_BLOCKSIZE as isize)) as libc::c_long
            as libc::c_int;
        curbuf = a.offset(SS_BLOCKSIZE as isize);
        if curbufsize <= bufsize {
            curbufsize = bufsize;
            curbuf = buf;
        }
        b = a;
        k = SS_BLOCKSIZE;
        j = i;
        while j & 1 as libc::c_int != 0 {
            ss_swapmerge(
                T,
                PA,
                b.offset(-(k as isize)),
                b,
                b.offset(k as isize),
                curbuf,
                curbufsize,
                depth,
            );
            b = b.offset(-(k as isize));
            k <<= 1 as libc::c_int;
            j >>= 1 as libc::c_int;
        }
        a = a.offset(SS_BLOCKSIZE as isize);
        i += 1;
    }
    ss_mintrosort(T, PA, a, middle, depth);
    k = SS_BLOCKSIZE;
    while i != 0 as libc::c_int {
        if i & 1 as libc::c_int != 0 {
            ss_swapmerge(T, PA, a.offset(-(k as isize)), a, middle, buf, bufsize, depth);
            a = a.offset(-(k as isize));
        }
        k <<= 1 as libc::c_int;
        i >>= 1 as libc::c_int;
    }
    if limit != 0 as libc::c_int {
        ss_mintrosort(T, PA, middle, last, depth);
        ss_inplacemerge(T, PA, first, middle, last, depth);
    }
    if lastsuffix != 0 as libc::c_int {
        let mut PAi: [libc::c_int; 2] = [0; 2];
        PAi[0 as libc::c_int
            as usize] = *PA.offset(*first.offset(-(1 as libc::c_int as isize)) as isize);
        PAi[1 as libc::c_int as usize] = n - 2 as libc::c_int;
        a = first;
        i = *first.offset(-(1 as libc::c_int as isize));
        while a < last
            && (*a < 0 as libc::c_int
                || (0 as libc::c_int)
                    < ss_compare(
                        T,
                        &mut *PAi.as_mut_ptr().offset(0 as libc::c_int as isize),
                        PA.offset(*a as isize),
                        depth,
                    ))
        {
            *a.offset(-(1 as libc::c_int as isize)) = *a;
            a = a.offset(1);
        }
        *a.offset(-(1 as libc::c_int as isize)) = i;
    }
}
#[inline]
unsafe extern "C" fn tr_ilg(mut n: libc::c_int) -> libc::c_int {
    return if n as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        if n as libc::c_uint & 0xff000000 as libc::c_uint != 0 {
            24 as libc::c_int
                + lg_table[(n >> 24 as libc::c_int & 0xff as libc::c_int) as usize]
        } else {
            16 as libc::c_int
                + lg_table[(n >> 16 as libc::c_int & 0xff as libc::c_int) as usize]
        }
    } else if n & 0xff00 as libc::c_int != 0 {
        8 as libc::c_int
            + lg_table[(n >> 8 as libc::c_int & 0xff as libc::c_int) as usize]
    } else {
        0 as libc::c_int
            + lg_table[(n >> 0 as libc::c_int & 0xff as libc::c_int) as usize]
    };
}
unsafe extern "C" fn tr_insertionsort(
    mut ISAd: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
) {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    a = first.offset(1 as libc::c_int as isize);
    while a < last {
        t = *a;
        b = a.offset(-(1 as libc::c_int as isize));
        loop {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if !(0 as libc::c_int > r) {
                break;
            }
            loop {
                *b.offset(1 as libc::c_int as isize) = *b;
                b = b.offset(-1);
                if !(first <= b && *b < 0 as libc::c_int) {
                    break;
                }
            }
            if b < first {
                break;
            }
        }
        if r == 0 as libc::c_int {
            *b = !*b;
        }
        *b.offset(1 as libc::c_int as isize) = t;
        a = a.offset(1);
    }
}
#[inline]
unsafe extern "C" fn tr_fixdown(
    mut ISAd: *const libc::c_int,
    mut SA: *mut libc::c_int,
    mut i: libc::c_int,
    mut size: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    v = *SA.offset(i as isize);
    c = *ISAd.offset(v as isize);
    loop {
        j = 2 as libc::c_int * i + 1 as libc::c_int;
        if !(j < size) {
            break;
        }
        let fresh54 = j;
        j = j + 1;
        k = fresh54;
        d = *ISAd.offset(*SA.offset(k as isize) as isize);
        e = *ISAd.offset(*SA.offset(j as isize) as isize);
        if d < e {
            k = j;
            d = e;
        }
        if d <= c {
            break;
        }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k;
    }
    *SA.offset(i as isize) = v;
}
unsafe extern "C" fn tr_heapsort(
    mut ISAd: *const libc::c_int,
    mut SA: *mut libc::c_int,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    m = size;
    if size % 2 as libc::c_int == 0 as libc::c_int {
        m -= 1;
        if *ISAd.offset(*SA.offset((m / 2 as libc::c_int) as isize) as isize)
            < *ISAd.offset(*SA.offset(m as isize) as isize)
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2 as libc::c_int) as isize);
            *SA.offset((m / 2 as libc::c_int) as isize) = t;
        }
    }
    i = m / 2 as libc::c_int - 1 as libc::c_int;
    while 0 as libc::c_int <= i {
        tr_fixdown(ISAd, SA, i, m);
        i -= 1;
    }
    if size % 2 as libc::c_int == 0 as libc::c_int {
        t = *SA.offset(0 as libc::c_int as isize);
        *SA.offset(0 as libc::c_int as isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        tr_fixdown(ISAd, SA, 0 as libc::c_int, m);
    }
    i = m - 1 as libc::c_int;
    while (0 as libc::c_int) < i {
        t = *SA.offset(0 as libc::c_int as isize);
        *SA.offset(0 as libc::c_int as isize) = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0 as libc::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn tr_median3(
    mut ISAd: *const libc::c_int,
    mut v1: *mut libc::c_int,
    mut v2: *mut libc::c_int,
    mut v3: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut t = 0 as *mut libc::c_int;
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v2 as isize) {
        t = v1;
        v1 = v2;
        v2 = t;
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
            return v1
        } else {
            return v3
        }
    }
    return v2;
}
#[inline]
unsafe extern "C" fn tr_median5(
    mut ISAd: *const libc::c_int,
    mut v1: *mut libc::c_int,
    mut v2: *mut libc::c_int,
    mut v3: *mut libc::c_int,
    mut v4: *mut libc::c_int,
    mut v5: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut t = 0 as *mut libc::c_int;
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        t = v2;
        v2 = v3;
        v3 = t;
    }
    if *ISAd.offset(*v4 as isize) > *ISAd.offset(*v5 as isize) {
        t = v4;
        v4 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v4 as isize) {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
        t = v1;
        v1 = v3;
        v3 = t;
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v4 as isize) {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v3 as isize) > *ISAd.offset(*v4 as isize) {
        return v4;
    }
    return v3;
}
#[inline]
unsafe extern "C" fn tr_pivot(
    mut ISAd: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut middle = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    t = last.offset_from(first) as libc::c_long as libc::c_int;
    middle = first.offset((t / 2 as libc::c_int) as isize);
    if t <= 512 as libc::c_int {
        if t <= 32 as libc::c_int {
            return tr_median3(
                ISAd,
                first,
                middle,
                last.offset(-(1 as libc::c_int as isize)),
            )
        } else {
            t >>= 2 as libc::c_int;
            return tr_median5(
                ISAd,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as libc::c_int as isize)).offset(-(t as isize)),
                last.offset(-(1 as libc::c_int as isize)),
            );
        }
    }
    t >>= 3 as libc::c_int;
    first = tr_median3(
        ISAd,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as libc::c_int) as isize),
    );
    middle = tr_median3(
        ISAd,
        middle.offset(-(t as isize)),
        middle,
        middle.offset(t as isize),
    );
    last = tr_median3(
        ISAd,
        last
            .offset(-(1 as libc::c_int as isize))
            .offset(-((t << 1 as libc::c_int) as isize)),
        last.offset(-(1 as libc::c_int as isize)).offset(-(t as isize)),
        last.offset(-(1 as libc::c_int as isize)),
    );
    return tr_median3(ISAd, first, middle, last);
}
#[inline]
unsafe extern "C" fn trbudget_init(
    mut budget: *mut trbudget_t,
    mut chance: libc::c_int,
    mut incval: libc::c_int,
) {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
#[inline]
unsafe extern "C" fn trbudget_check(
    mut budget: *mut trbudget_t,
    mut size: libc::c_int,
) -> libc::c_int {
    if size <= (*budget).remain {
        (*budget).remain -= size;
        return 1 as libc::c_int;
    }
    if (*budget).chance == 0 as libc::c_int {
        (*budget).count += size;
        return 0 as libc::c_int;
    }
    (*budget).remain += (*budget).incval - size;
    (*budget).chance -= 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn tr_partition(
    mut ISAd: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut middle: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut pa: *mut *mut libc::c_int,
    mut pb: *mut *mut libc::c_int,
    mut v: libc::c_int,
) {
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut c = 0 as *mut libc::c_int;
    let mut d = 0 as *mut libc::c_int;
    let mut e = 0 as *mut libc::c_int;
    let mut f = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut x = 0 as libc::c_int;
    b = middle.offset(-(1 as libc::c_int as isize));
    loop {
        b = b.offset(1);
        if !(b < last
            && {
                x = *ISAd.offset(*b as isize);
                x == v
            })
        {
            break;
        }
    }
    a = b;
    if a < last && x < v {
        loop {
            b = b.offset(1);
            if !(b < last
                && {
                    x = *ISAd.offset(*b as isize);
                    x <= v
                })
            {
                break;
            }
            if x == v {
                t = *b;
                *b = *a;
                *a = t;
                a = a.offset(1);
            }
        }
    }
    c = last;
    loop {
        c = c.offset(-1);
        if !(b < c
            && {
                x = *ISAd.offset(*c as isize);
                x == v
            })
        {
            break;
        }
    }
    d = c;
    if b < d && x > v {
        loop {
            c = c.offset(-1);
            if !(b < c
                && {
                    x = *ISAd.offset(*c as isize);
                    x >= v
                })
            {
                break;
            }
            if x == v {
                t = *c;
                *c = *d;
                *d = t;
                d = d.offset(-1);
            }
        }
    }
    while b < c {
        t = *b;
        *b = *c;
        *c = t;
        loop {
            b = b.offset(1);
            if !(b < c
                && {
                    x = *ISAd.offset(*b as isize);
                    x <= v
                })
            {
                break;
            }
            if x == v {
                t = *b;
                *b = *a;
                *a = t;
                a = a.offset(1);
            }
        }
        loop {
            c = c.offset(-1);
            if !(b < c
                && {
                    x = *ISAd.offset(*c as isize);
                    x >= v
                })
            {
                break;
            }
            if x == v {
                t = *c;
                *c = *d;
                *d = t;
                d = d.offset(-1);
            }
        }
    }
    if a <= d {
        c = b.offset(-(1 as libc::c_int as isize));
        s = a.offset_from(first) as libc::c_long as libc::c_int;
        t = b.offset_from(a) as libc::c_long as libc::c_int;
        if s > t {
            s = t;
        }
        e = first;
        f = b.offset(-(s as isize));
        while (0 as libc::c_int) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1);
            f = f.offset(1);
        }
        s = d.offset_from(c) as libc::c_long as libc::c_int;
        t = (last.offset_from(d) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if s > t {
            s = t;
        }
        e = b;
        f = last.offset(-(s as isize));
        while (0 as libc::c_int) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1);
            f = f.offset(1);
        }
        first = first.offset(b.offset_from(a) as libc::c_long as isize);
        last = last.offset(-(d.offset_from(c) as libc::c_long as isize));
    }
    *pa = first;
    *pb = last;
}
unsafe extern "C" fn tr_copy(
    mut ISA: *mut libc::c_int,
    mut SA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut c = 0 as *mut libc::c_int;
    let mut d = 0 as *mut libc::c_int;
    let mut e = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    v = (b.offset_from(SA) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    c = first;
    d = a.offset(-(1 as libc::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as libc::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as libc::c_long as libc::c_int;
        }
        c = c.offset(1);
    }
    c = last.offset(-(1 as libc::c_int as isize));
    e = d.offset(1 as libc::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as libc::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as libc::c_long as libc::c_int;
        }
        c = c.offset(-1);
    }
}
unsafe extern "C" fn tr_partialcopy(
    mut ISA: *mut libc::c_int,
    mut SA: *const libc::c_int,
    mut first: *mut libc::c_int,
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut depth: libc::c_int,
) {
    let mut c = 0 as *mut libc::c_int;
    let mut d = 0 as *mut libc::c_int;
    let mut e = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    let mut lastrank: libc::c_int = 0;
    let mut newrank = -(1 as libc::c_int);
    v = (b.offset_from(SA) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    lastrank = -(1 as libc::c_int);
    c = first;
    d = a.offset(-(1 as libc::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as libc::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as libc::c_long as libc::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(1);
    }
    lastrank = -(1 as libc::c_int);
    e = d;
    while first <= e {
        rank = *ISA.offset(*e as isize);
        if lastrank != rank {
            lastrank = rank;
            newrank = e.offset_from(SA) as libc::c_long as libc::c_int;
        }
        if newrank != rank {
            *ISA.offset(*e as isize) = newrank;
        }
        e = e.offset(-1);
    }
    lastrank = -(1 as libc::c_int);
    c = last.offset(-(1 as libc::c_int as isize));
    e = d.offset(1 as libc::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as libc::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as libc::c_long as libc::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(-1);
    }
}
unsafe extern "C" fn tr_introsort(
    mut ISA: *mut libc::c_int,
    mut ISAd: *const libc::c_int,
    mut SA: *mut libc::c_int,
    mut first: *mut libc::c_int,
    mut last: *mut libc::c_int,
    mut budget: *mut trbudget_t,
) {
    let mut stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
        a: 0 as *const libc::c_int,
        b: 0 as *mut libc::c_int,
        c: 0 as *mut libc::c_int,
        d: 0,
        e: 0,
    }; 64];
    let mut a = 0 as *mut libc::c_int;
    let mut b = 0 as *mut libc::c_int;
    let mut c = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x = 0 as libc::c_int;
    let mut incr = ISAd.offset_from(ISA) as libc::c_long as libc::c_int;
    let mut limit: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut trlink = -(1 as libc::c_int);
    ssize = 0 as libc::c_int;
    limit = tr_ilg(last.offset_from(first) as libc::c_long as libc::c_int);
    loop {
        if limit < 0 as libc::c_int {
            if limit == -(1 as libc::c_int) {
                tr_partition(
                    ISAd.offset(-(incr as isize)),
                    first,
                    first,
                    last,
                    &mut a,
                    &mut b,
                    (last.offset_from(SA) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int,
                );
                if a < last {
                    c = first;
                    v = (a.offset_from(SA) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                }
                if b < last {
                    c = a;
                    v = (b.offset_from(SA) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                }
                if (1 as libc::c_int as libc::c_long) < b.offset_from(a) as libc::c_long
                {
                    if ssize < 64 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1204 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = 0 as *const libc::c_int;
                    stack[ssize as usize].b = a;
                    stack[ssize as usize].c = b;
                    stack[ssize as usize].d = 0 as libc::c_int;
                    let fresh55 = ssize;
                    ssize = ssize + 1;
                    stack[fresh55 as usize].e = 0 as libc::c_int;
                    if ssize < 64 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1205 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = ISAd.offset(-(incr as isize));
                    stack[ssize as usize].b = first;
                    stack[ssize as usize].c = last;
                    stack[ssize as usize].d = -(2 as libc::c_int);
                    let fresh56 = ssize;
                    ssize = ssize + 1;
                    stack[fresh56 as usize].e = trlink;
                    trlink = ssize - 2 as libc::c_int;
                }
                if a.offset_from(first) as libc::c_long
                    <= last.offset_from(b) as libc::c_long
                {
                    if (1 as libc::c_int as libc::c_long)
                        < a.offset_from(first) as libc::c_long
                    {
                        if ssize < 64 as libc::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                1210 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[libc::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        stack[ssize as usize].a = ISAd;
                        stack[ssize as usize].b = b;
                        stack[ssize as usize].c = last;
                        stack[ssize as usize]
                            .d = tr_ilg(
                            last.offset_from(b) as libc::c_long as libc::c_int,
                        );
                        let fresh57 = ssize;
                        ssize = ssize + 1;
                        stack[fresh57 as usize].e = trlink;
                        last = a;
                        limit = tr_ilg(
                            a.offset_from(first) as libc::c_long as libc::c_int,
                        );
                    } else if (1 as libc::c_int as libc::c_long)
                        < last.offset_from(b) as libc::c_long
                    {
                        first = b;
                        limit = tr_ilg(
                            last.offset_from(b) as libc::c_long as libc::c_int,
                        );
                    } else {
                        if 0 as libc::c_int <= ssize {} else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const libc::c_char,
                                1215 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[libc::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if ssize == 0 as libc::c_int {
                            return;
                        }
                        ssize -= 1;
                        ISAd = stack[ssize as usize].a;
                        first = stack[ssize as usize].b;
                        last = stack[ssize as usize].c;
                        limit = stack[ssize as usize].d;
                        trlink = stack[ssize as usize].e;
                    }
                } else if (1 as libc::c_int as libc::c_long)
                    < last.offset_from(b) as libc::c_long
                {
                    if ssize < 64 as libc::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1219 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    stack[ssize as usize].a = ISAd;
                    stack[ssize as usize].b = first;
                    stack[ssize as usize].c = a;
                    stack[ssize as usize]
                        .d = tr_ilg(a.offset_from(first) as libc::c_long as libc::c_int);
                    let fresh58 = ssize;
                    ssize = ssize + 1;
                    stack[fresh58 as usize].e = trlink;
                    first = b;
                    limit = tr_ilg(last.offset_from(b) as libc::c_long as libc::c_int);
                } else if (1 as libc::c_int as libc::c_long)
                    < a.offset_from(first) as libc::c_long
                {
                    last = a;
                    limit = tr_ilg(a.offset_from(first) as libc::c_long as libc::c_int);
                } else {
                    if 0 as libc::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1224 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 as libc::c_int {
                        return;
                    }
                    ssize -= 1;
                    ISAd = stack[ssize as usize].a;
                    first = stack[ssize as usize].b;
                    last = stack[ssize as usize].c;
                    limit = stack[ssize as usize].d;
                    trlink = stack[ssize as usize].e;
                }
            } else if limit == -(2 as libc::c_int) {
                ssize -= 1;
                a = stack[ssize as usize].b;
                b = stack[ssize as usize].c;
                if stack[ssize as usize].d == 0 as libc::c_int {
                    tr_copy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as libc::c_long as libc::c_int,
                    );
                } else {
                    if 0 as libc::c_int <= trlink {
                        stack[trlink as usize].d = -(1 as libc::c_int);
                    }
                    tr_partialcopy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as libc::c_long as libc::c_int,
                    );
                }
                if 0 as libc::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const libc::c_char,
                        1236 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 73],
                            &[libc::c_char; 73],
                        >(
                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if ssize == 0 as libc::c_int {
                    return;
                }
                ssize -= 1;
                ISAd = stack[ssize as usize].a;
                first = stack[ssize as usize].b;
                last = stack[ssize as usize].c;
                limit = stack[ssize as usize].d;
                trlink = stack[ssize as usize].e;
            } else {
                if 0 as libc::c_int <= *first {
                    a = first;
                    loop {
                        *ISA
                            .offset(
                                *a as isize,
                            ) = a.offset_from(SA) as libc::c_long as libc::c_int;
                        a = a.offset(1);
                        if !(a < last && 0 as libc::c_int <= *a) {
                            break;
                        }
                    }
                    first = a;
                }
                if first < last {
                    a = first;
                    loop {
                        *a = !*a;
                        a = a.offset(1);
                        if !(*a < 0 as libc::c_int) {
                            break;
                        }
                    }
                    next = if *ISA.offset(*a as isize) != *ISAd.offset(*a as isize) {
                        tr_ilg(
                            (a.offset_from(first) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as libc::c_int,
                        )
                    } else {
                        -(1 as libc::c_int)
                    };
                    a = a.offset(1);
                    if a < last {
                        b = first;
                        v = (a.offset_from(SA) as libc::c_long
                            - 1 as libc::c_int as libc::c_long) as libc::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.offset(1);
                        }
                    }
                    if trbudget_check(
                        budget,
                        a.offset_from(first) as libc::c_long as libc::c_int,
                    ) != 0
                    {
                        if a.offset_from(first) as libc::c_long
                            <= last.offset_from(a) as libc::c_long
                        {
                            if ssize < 64 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1252 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = a;
                            stack[ssize as usize].c = last;
                            stack[ssize as usize].d = -(3 as libc::c_int);
                            let fresh59 = ssize;
                            ssize = ssize + 1;
                            stack[fresh59 as usize].e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        } else if (1 as libc::c_int as libc::c_long)
                            < last.offset_from(a) as libc::c_long
                        {
                            if ssize < 64 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1256 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = ISAd.offset(incr as isize);
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = next;
                            let fresh60 = ssize;
                            ssize = ssize + 1;
                            stack[fresh60 as usize].e = trlink;
                            first = a;
                            limit = -(3 as libc::c_int);
                        } else {
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        }
                    } else {
                        if 0 as libc::c_int <= trlink {
                            stack[trlink as usize].d = -(1 as libc::c_int);
                        }
                        if (1 as libc::c_int as libc::c_long)
                            < last.offset_from(a) as libc::c_long
                        {
                            first = a;
                            limit = -(3 as libc::c_int);
                        } else {
                            if 0 as libc::c_int <= ssize {} else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1267 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if ssize == 0 as libc::c_int {
                                return;
                            }
                            ssize -= 1;
                            ISAd = stack[ssize as usize].a;
                            first = stack[ssize as usize].b;
                            last = stack[ssize as usize].c;
                            limit = stack[ssize as usize].d;
                            trlink = stack[ssize as usize].e;
                        }
                    }
                } else {
                    if 0 as libc::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1271 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 as libc::c_int {
                        return;
                    }
                    ssize -= 1;
                    ISAd = stack[ssize as usize].a;
                    first = stack[ssize as usize].b;
                    last = stack[ssize as usize].c;
                    limit = stack[ssize as usize].d;
                    trlink = stack[ssize as usize].e;
                }
            }
        } else if last.offset_from(first) as libc::c_long
            <= TR_INSERTIONSORT_THRESHOLD as libc::c_long
        {
            tr_insertionsort(ISAd, first, last);
            limit = -(3 as libc::c_int);
        } else {
            let fresh61 = limit;
            limit = limit - 1;
            if fresh61 == 0 as libc::c_int {
                tr_heapsort(
                    ISAd,
                    first,
                    last.offset_from(first) as libc::c_long as libc::c_int,
                );
                a = last.offset(-(1 as libc::c_int as isize));
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.offset(-(1 as libc::c_int as isize));
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.offset(-1);
                    }
                    a = b;
                }
                limit = -(3 as libc::c_int);
            } else {
                a = tr_pivot(ISAd, first, last);
                t = *first;
                *first = *a;
                *a = t;
                v = *ISAd.offset(*first as isize);
                tr_partition(
                    ISAd,
                    first,
                    first.offset(1 as libc::c_int as isize),
                    last,
                    &mut a,
                    &mut b,
                    v,
                );
                if last.offset_from(first) as libc::c_long
                    != b.offset_from(a) as libc::c_long
                {
                    next = if *ISA.offset(*a as isize) != v {
                        tr_ilg(b.offset_from(a) as libc::c_long as libc::c_int)
                    } else {
                        -(1 as libc::c_int)
                    };
                    c = first;
                    v = (a.offset_from(SA) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                    if b < last {
                        c = a;
                        v = (b.offset_from(SA) as libc::c_long
                            - 1 as libc::c_int as libc::c_long) as libc::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.offset(1);
                        }
                    }
                    if (1 as libc::c_int as libc::c_long)
                        < b.offset_from(a) as libc::c_long
                        && trbudget_check(
                            budget,
                            b.offset_from(a) as libc::c_long as libc::c_int,
                        ) != 0
                    {
                        if a.offset_from(first) as libc::c_long
                            <= last.offset_from(b) as libc::c_long
                        {
                            if last.offset_from(b) as libc::c_long
                                <= b.offset_from(a) as libc::c_long
                            {
                                if (1 as libc::c_int as libc::c_long)
                                    < a.offset_from(first) as libc::c_long
                                {
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1311 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh62 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh62 as usize].e = trlink;
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1312 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh63 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh63 as usize].e = trlink;
                                    last = a;
                                } else if (1 as libc::c_int as libc::c_long)
                                    < last.offset_from(b) as libc::c_long
                                {
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1315 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh64 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh64 as usize].e = trlink;
                                    first = b;
                                } else {
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else if a.offset_from(first) as libc::c_long
                                <= b.offset_from(a) as libc::c_long
                            {
                                if (1 as libc::c_int as libc::c_long)
                                    < a.offset_from(first) as libc::c_long
                                {
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1322 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh65 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh65 as usize].e = trlink;
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1323 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh66 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh66 as usize].e = trlink;
                                    last = a;
                                } else {
                                    if ssize < 64 as libc::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const libc::c_char,
                                            1326 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[libc::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh67 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh67 as usize].e = trlink;
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1330 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = b;
                                stack[ssize as usize].c = last;
                                stack[ssize as usize].d = limit;
                                let fresh68 = ssize;
                                ssize = ssize + 1;
                                stack[fresh68 as usize].e = trlink;
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1331 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh69 = ssize;
                                ssize = ssize + 1;
                                stack[fresh69 as usize].e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if a.offset_from(first) as libc::c_long
                            <= b.offset_from(a) as libc::c_long
                        {
                            if (1 as libc::c_int as libc::c_long)
                                < last.offset_from(b) as libc::c_long
                            {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1337 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh70 = ssize;
                                ssize = ssize + 1;
                                stack[fresh70 as usize].e = trlink;
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1338 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh71 = ssize;
                                ssize = ssize + 1;
                                stack[fresh71 as usize].e = trlink;
                                first = b;
                            } else if (1 as libc::c_int as libc::c_long)
                                < a.offset_from(first) as libc::c_long
                            {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1341 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh72 = ssize;
                                ssize = ssize + 1;
                                stack[fresh72 as usize].e = trlink;
                                last = a;
                            } else {
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if last.offset_from(b) as libc::c_long
                            <= b.offset_from(a) as libc::c_long
                        {
                            if (1 as libc::c_int as libc::c_long)
                                < last.offset_from(b) as libc::c_long
                            {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1348 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh73 = ssize;
                                ssize = ssize + 1;
                                stack[fresh73 as usize].e = trlink;
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1349 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh74 = ssize;
                                ssize = ssize + 1;
                                stack[fresh74 as usize].e = trlink;
                                first = b;
                            } else {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1352 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh75 = ssize;
                                ssize = ssize + 1;
                                stack[fresh75 as usize].e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else {
                            if ssize < 64 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1356 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = limit;
                            let fresh76 = ssize;
                            ssize = ssize + 1;
                            stack[fresh76 as usize].e = trlink;
                            if ssize < 64 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1357 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = b;
                            stack[ssize as usize].c = last;
                            stack[ssize as usize].d = limit;
                            let fresh77 = ssize;
                            ssize = ssize + 1;
                            stack[fresh77 as usize].e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            first = a;
                            last = b;
                            limit = next;
                        }
                    } else {
                        if (1 as libc::c_int as libc::c_long)
                            < b.offset_from(a) as libc::c_long
                            && 0 as libc::c_int <= trlink
                        {
                            stack[trlink as usize].d = -(1 as libc::c_int);
                        }
                        if a.offset_from(first) as libc::c_long
                            <= last.offset_from(b) as libc::c_long
                        {
                            if (1 as libc::c_int as libc::c_long)
                                < a.offset_from(first) as libc::c_long
                            {
                                if ssize < 64 as libc::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1365 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = b;
                                stack[ssize as usize].c = last;
                                stack[ssize as usize].d = limit;
                                let fresh78 = ssize;
                                ssize = ssize + 1;
                                stack[fresh78 as usize].e = trlink;
                                last = a;
                            } else if (1 as libc::c_int as libc::c_long)
                                < last.offset_from(b) as libc::c_long
                            {
                                first = b;
                            } else {
                                if 0 as libc::c_int <= ssize {} else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const libc::c_char,
                                        1370 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                if ssize == 0 as libc::c_int {
                                    return;
                                }
                                ssize -= 1;
                                ISAd = stack[ssize as usize].a;
                                first = stack[ssize as usize].b;
                                last = stack[ssize as usize].c;
                                limit = stack[ssize as usize].d;
                                trlink = stack[ssize as usize].e;
                            }
                        } else if (1 as libc::c_int as libc::c_long)
                            < last.offset_from(b) as libc::c_long
                        {
                            if ssize < 64 as libc::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1374 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = limit;
                            let fresh79 = ssize;
                            ssize = ssize + 1;
                            stack[fresh79 as usize].e = trlink;
                            first = b;
                        } else if (1 as libc::c_int as libc::c_long)
                            < a.offset_from(first) as libc::c_long
                        {
                            last = a;
                        } else {
                            if 0 as libc::c_int <= ssize {} else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const libc::c_char,
                                    1379 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if ssize == 0 as libc::c_int {
                                return;
                            }
                            ssize -= 1;
                            ISAd = stack[ssize as usize].a;
                            first = stack[ssize as usize].b;
                            last = stack[ssize as usize].c;
                            limit = stack[ssize as usize].d;
                            trlink = stack[ssize as usize].e;
                        }
                    }
                } else if trbudget_check(
                    budget,
                    last.offset_from(first) as libc::c_long as libc::c_int,
                ) != 0
                {
                    limit = tr_ilg(
                        last.offset_from(first) as libc::c_long as libc::c_int,
                    );
                    ISAd = ISAd.offset(incr as isize);
                } else {
                    if 0 as libc::c_int <= trlink {
                        stack[trlink as usize].d = -(1 as libc::c_int);
                    }
                    if 0 as libc::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1388 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 as libc::c_int {
                        return;
                    }
                    ssize -= 1;
                    ISAd = stack[ssize as usize].a;
                    first = stack[ssize as usize].b;
                    last = stack[ssize as usize].c;
                    limit = stack[ssize as usize].d;
                    trlink = stack[ssize as usize].e;
                }
            }
        }
    };
}
unsafe extern "C" fn trsort(
    mut ISA: *mut libc::c_int,
    mut SA: *mut libc::c_int,
    mut n: libc::c_int,
    mut depth: libc::c_int,
) {
    let mut ISAd = 0 as *mut libc::c_int;
    let mut first = 0 as *mut libc::c_int;
    let mut last = 0 as *mut libc::c_int;
    let mut budget = trbudget_t {
        chance: 0,
        remain: 0,
        incval: 0,
        count: 0,
    };
    let mut t: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut unsorted: libc::c_int = 0;
    trbudget_init(&mut budget, tr_ilg(n) * 2 as libc::c_int / 3 as libc::c_int, n);
    ISAd = ISA.offset(depth as isize);
    while -n < *SA {
        first = SA;
        skip = 0 as libc::c_int;
        unsorted = 0 as libc::c_int;
        loop {
            t = *first;
            if t < 0 as libc::c_int {
                first = first.offset(-(t as isize));
                skip += t;
            } else {
                if skip != 0 as libc::c_int {
                    *first.offset(skip as isize) = skip;
                    skip = 0 as libc::c_int;
                }
                last = SA
                    .offset(*ISA.offset(t as isize) as isize)
                    .offset(1 as libc::c_int as isize);
                if (1 as libc::c_int as libc::c_long)
                    < last.offset_from(first) as libc::c_long
                {
                    budget.count = 0 as libc::c_int;
                    tr_introsort(ISA, ISAd, SA, first, last, &mut budget);
                    if budget.count != 0 as libc::c_int {
                        unsorted += budget.count;
                    } else {
                        skip = first.offset_from(last) as libc::c_long as libc::c_int;
                    }
                } else if last.offset_from(first) as libc::c_long
                    == 1 as libc::c_int as libc::c_long
                {
                    skip = -(1 as libc::c_int);
                }
                first = last;
            }
            if !(first < SA.offset(n as isize)) {
                break;
            }
        }
        if skip != 0 as libc::c_int {
            *first.offset(skip as isize) = skip;
        }
        if unsorted == 0 as libc::c_int {
            break;
        }
        ISAd = ISAd.offset(ISAd.offset_from(ISA) as libc::c_long as isize);
    }
}
unsafe extern "C" fn sort_typeBstar(
    mut T: *const libc::c_uchar,
    mut SA: *mut libc::c_int,
    mut bucket_A: *mut libc::c_int,
    mut bucket_B: *mut libc::c_int,
    mut n: libc::c_int,
    mut openMP: libc::c_int,
) -> libc::c_int {
    let mut PAb = 0 as *mut libc::c_int;
    let mut ISAb = 0 as *mut libc::c_int;
    let mut buf = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut bufsize: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < BUCKET_A_SIZE {
        *bucket_A.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < BUCKET_B_SIZE {
        *bucket_B.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    i = n - 1 as libc::c_int;
    m = n;
    c0 = *T.offset((n - 1 as libc::c_int) as isize) as libc::c_int;
    while 0 as libc::c_int <= i {
        loop {
            c1 = c0;
            let ref mut fresh80 = *bucket_A.offset(c1 as isize);
            *fresh80 += 1;
            i -= 1;
            if !(0 as libc::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as libc::c_int;
                    c0 >= c1
                })
            {
                break;
            }
        }
        if 0 as libc::c_int <= i {
            let ref mut fresh81 = *bucket_B
                .offset((c0 << 8 as libc::c_int | c1) as isize);
            *fresh81 += 1;
            m -= 1;
            *SA.offset(m as isize) = i;
            i -= 1;
            c1 = c0;
            while 0 as libc::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as libc::c_int;
                    c0 <= c1
                }
            {
                let ref mut fresh82 = *bucket_B
                    .offset((c1 << 8 as libc::c_int | c0) as isize);
                *fresh82 += 1;
                i -= 1;
                c1 = c0;
            }
        }
    }
    m = n - m;
    c0 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while c0 < ALPHABET_SIZE {
        t = i + *bucket_A.offset(c0 as isize);
        *bucket_A.offset(c0 as isize) = i + j;
        i = t + *bucket_B.offset((c0 << 8 as libc::c_int | c0) as isize);
        c1 = c0 + 1 as libc::c_int;
        while c1 < ALPHABET_SIZE {
            j += *bucket_B.offset((c0 << 8 as libc::c_int | c1) as isize);
            *bucket_B.offset((c0 << 8 as libc::c_int | c1) as isize) = j;
            i += *bucket_B.offset((c1 << 8 as libc::c_int | c0) as isize);
            c1 += 1;
        }
        c0 += 1;
    }
    if (0 as libc::c_int) < m {
        PAb = SA.offset(n as isize).offset(-(m as isize));
        ISAb = SA.offset(m as isize);
        i = m - 2 as libc::c_int;
        while 0 as libc::c_int <= i {
            t = *PAb.offset(i as isize);
            c0 = *T.offset(t as isize) as libc::c_int;
            c1 = *T.offset((t + 1 as libc::c_int) as isize) as libc::c_int;
            let ref mut fresh83 = *bucket_B
                .offset((c0 << 8 as libc::c_int | c1) as isize);
            *fresh83 -= 1;
            *SA.offset(*fresh83 as isize) = i;
            i -= 1;
        }
        t = *PAb.offset((m - 1 as libc::c_int) as isize);
        c0 = *T.offset(t as isize) as libc::c_int;
        c1 = *T.offset((t + 1 as libc::c_int) as isize) as libc::c_int;
        let ref mut fresh84 = *bucket_B.offset((c0 << 8 as libc::c_int | c1) as isize);
        *fresh84 -= 1;
        *SA.offset(*fresh84 as isize) = m - 1 as libc::c_int;
        buf = SA.offset(m as isize);
        bufsize = n - 2 as libc::c_int * m;
        c0 = ALPHABET_SIZE - 2 as libc::c_int;
        j = m;
        while (0 as libc::c_int) < j {
            c1 = ALPHABET_SIZE - 1 as libc::c_int;
            while c0 < c1 {
                i = *bucket_B.offset((c0 << 8 as libc::c_int | c1) as isize);
                if (1 as libc::c_int) < j - i {
                    sssort(
                        T,
                        PAb,
                        SA.offset(i as isize),
                        SA.offset(j as isize),
                        buf,
                        bufsize,
                        2 as libc::c_int,
                        n,
                        (*SA.offset(i as isize) == m - 1 as libc::c_int) as libc::c_int,
                    );
                }
                j = i;
                c1 -= 1;
            }
            c0 -= 1;
        }
        i = m - 1 as libc::c_int;
        while 0 as libc::c_int <= i {
            if 0 as libc::c_int <= *SA.offset(i as isize) {
                j = i;
                loop {
                    *ISAb.offset(*SA.offset(i as isize) as isize) = i;
                    i -= 1;
                    if !(0 as libc::c_int <= i
                        && 0 as libc::c_int <= *SA.offset(i as isize))
                    {
                        break;
                    }
                }
                *SA.offset((i + 1 as libc::c_int) as isize) = i - j;
                if i <= 0 as libc::c_int {
                    break;
                }
            }
            j = i;
            loop {
                let ref mut fresh85 = *SA.offset(i as isize);
                *fresh85 = !*SA.offset(i as isize);
                *ISAb.offset(*fresh85 as isize) = j;
                i -= 1;
                if !(*SA.offset(i as isize) < 0 as libc::c_int) {
                    break;
                }
            }
            *ISAb.offset(*SA.offset(i as isize) as isize) = j;
            i -= 1;
        }
        trsort(ISAb, SA, m, 1 as libc::c_int);
        i = n - 1 as libc::c_int;
        j = m;
        c0 = *T.offset((n - 1 as libc::c_int) as isize) as libc::c_int;
        while 0 as libc::c_int <= i {
            i -= 1;
            c1 = c0;
            while 0 as libc::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as libc::c_int;
                    c0 >= c1
                }
            {
                i -= 1;
                c1 = c0;
            }
            if 0 as libc::c_int <= i {
                t = i;
                i -= 1;
                c1 = c0;
                while 0 as libc::c_int <= i
                    && {
                        c0 = *T.offset(i as isize) as libc::c_int;
                        c0 <= c1
                    }
                {
                    i -= 1;
                    c1 = c0;
                }
                j -= 1;
                *SA
                    .offset(
                        *ISAb.offset(j as isize) as isize,
                    ) = if t == 0 as libc::c_int || (1 as libc::c_int) < t - i {
                    t
                } else {
                    !t
                };
            }
        }
        *bucket_B
            .offset(
                ((256 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int
                    | 256 as libc::c_int - 1 as libc::c_int) as isize,
            ) = n;
        c0 = ALPHABET_SIZE - 2 as libc::c_int;
        k = m - 1 as libc::c_int;
        while 0 as libc::c_int <= c0 {
            i = *bucket_A.offset((c0 + 1 as libc::c_int) as isize) - 1 as libc::c_int;
            c1 = ALPHABET_SIZE - 1 as libc::c_int;
            while c0 < c1 {
                t = i - *bucket_B.offset((c1 << 8 as libc::c_int | c0) as isize);
                *bucket_B.offset((c1 << 8 as libc::c_int | c0) as isize) = i;
                i = t;
                j = *bucket_B.offset((c0 << 8 as libc::c_int | c1) as isize);
                while j <= k {
                    *SA.offset(i as isize) = *SA.offset(k as isize);
                    i -= 1;
                    k -= 1;
                }
                c1 -= 1;
            }
            *bucket_B
                .offset(
                    (c0 << 8 as libc::c_int | c0 + 1 as libc::c_int) as isize,
                ) = i - *bucket_B.offset((c0 << 8 as libc::c_int | c0) as isize)
                + 1 as libc::c_int;
            *bucket_B.offset((c0 << 8 as libc::c_int | c0) as isize) = i;
            c0 -= 1;
        }
    }
    return m;
}
unsafe extern "C" fn construct_SA(
    mut T: *const libc::c_uchar,
    mut SA: *mut libc::c_int,
    mut bucket_A: *mut libc::c_int,
    mut bucket_B: *mut libc::c_int,
    mut n: libc::c_int,
    mut m: libc::c_int,
) {
    let mut i = 0 as *mut libc::c_int;
    let mut j = 0 as *mut libc::c_int;
    let mut k = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if (0 as libc::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as libc::c_int;
        while 0 as libc::c_int <= c1 {
            i = SA
                .offset(
                    *bucket_B
                        .offset(
                            (c1 << 8 as libc::c_int | c1 + 1 as libc::c_int) as isize,
                        ) as isize,
                );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as libc::c_int) as isize) as isize)
                .offset(-(1 as libc::c_int as isize));
            k = NULL as *mut libc::c_int;
            c2 = -(1 as libc::c_int);
            while i <= j {
                s = *j;
                if (0 as libc::c_int) < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {} else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1630 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if (s + 1 as libc::c_int) < n
                        && *T.offset(s as isize) as libc::c_int
                            <= *T.offset((s + 1 as libc::c_int) as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1631 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                        <= *T.offset(s as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1632 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *j = !s;
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    if (0 as libc::c_int) < s
                        && *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as libc::c_int <= c2 {
                            *bucket_B
                                .offset(
                                    (c1 << 8 as libc::c_int | c2) as isize,
                                ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(
                                *bucket_B.offset((c1 << 8 as libc::c_int | c2) as isize)
                                    as isize,
                            );
                    }
                    if k < j {} else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1640 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {} else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1640 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh86 = k;
                    k = k.offset(-1);
                    *fresh86 = s;
                } else {
                    if s == 0 as libc::c_int
                        && *T.offset(s as isize) as libc::c_int == c1
                        || s < 0 as libc::c_int
                    {} else {
                        __assert_fail(
                            b"((s == 0) && (T[s] == c1)) || (s < 0)\0" as *const u8
                                as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1643 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *j = !s;
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1 as libc::c_int) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh87 = k;
    k = k.offset(1);
    *fresh87 = if (*T.offset((n - 2 as libc::c_int) as isize) as libc::c_int) < c2 {
        !(n - 1 as libc::c_int)
    } else {
        n - 1 as libc::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    while i < j {
        s = *i;
        if (0 as libc::c_int) < s {
            if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                >= *T.offset(s as isize) as libc::c_int
            {} else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1657 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            if s == 0 as libc::c_int
                || (*T.offset((s - 1 as libc::c_int) as isize) as libc::c_int) < c0
            {
                s = !s;
            }
            if c0 != c2 {
                *bucket_A
                    .offset(
                        c2 as isize,
                    ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {} else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1664 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            let fresh88 = k;
            k = k.offset(1);
            *fresh88 = s;
        } else {
            if s < 0 as libc::c_int {} else {
                __assert_fail(
                    b"s < 0\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1667 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            *i = !s;
        }
        i = i.offset(1);
    }
}
unsafe extern "C" fn construct_BWT(
    mut T: *const libc::c_uchar,
    mut SA: *mut libc::c_int,
    mut bucket_A: *mut libc::c_int,
    mut bucket_B: *mut libc::c_int,
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut i = 0 as *mut libc::c_int;
    let mut j = 0 as *mut libc::c_int;
    let mut k = 0 as *mut libc::c_int;
    let mut orig = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if (0 as libc::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as libc::c_int;
        while 0 as libc::c_int <= c1 {
            i = SA
                .offset(
                    *bucket_B
                        .offset(
                            (c1 << 8 as libc::c_int | c1 + 1 as libc::c_int) as isize,
                        ) as isize,
                );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as libc::c_int) as isize) as isize)
                .offset(-(1 as libc::c_int as isize));
            k = NULL as *mut libc::c_int;
            c2 = -(1 as libc::c_int);
            while i <= j {
                s = *j;
                if (0 as libc::c_int) < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {} else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1694 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if (s + 1 as libc::c_int) < n
                        && *T.offset(s as isize) as libc::c_int
                            <= *T.offset((s + 1 as libc::c_int) as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1695 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                        <= *T.offset(s as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1696 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    *j = !c0;
                    if (0 as libc::c_int) < s
                        && *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as libc::c_int <= c2 {
                            *bucket_B
                                .offset(
                                    (c1 << 8 as libc::c_int | c2) as isize,
                                ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(
                                *bucket_B.offset((c1 << 8 as libc::c_int | c2) as isize)
                                    as isize,
                            );
                    }
                    if k < j {} else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1704 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {} else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1704 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh89 = k;
                    k = k.offset(-1);
                    *fresh89 = s;
                } else if s != 0 as libc::c_int {
                    *j = !s;
                } else if *T.offset(s as isize) as libc::c_int == c1 {} else {
                    __assert_fail(
                        b"T[s] == c1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const libc::c_char,
                        1710 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[libc::c_char; 72],
                        >(
                            b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1 as libc::c_int) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh90 = k;
    k = k.offset(1);
    *fresh90 = if (*T.offset((n - 2 as libc::c_int) as isize) as libc::c_int) < c2 {
        !(*T.offset((n - 2 as libc::c_int) as isize) as libc::c_int)
    } else {
        n - 1 as libc::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as libc::c_int) < s {
            if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                >= *T.offset(s as isize) as libc::c_int
            {} else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1724 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            *i = c0;
            if (0 as libc::c_int) < s
                && (*T.offset((s - 1 as libc::c_int) as isize) as libc::c_int) < c0
            {
                s = !(*T.offset((s - 1 as libc::c_int) as isize) as libc::c_int);
            }
            if c0 != c2 {
                *bucket_A
                    .offset(
                        c2 as isize,
                    ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {} else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1732 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            let fresh91 = k;
            k = k.offset(1);
            *fresh91 = s;
        } else if s != 0 as libc::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
    }
    return orig.offset_from(SA) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn construct_BWT_indexes(
    mut T: *const libc::c_uchar,
    mut SA: *mut libc::c_int,
    mut bucket_A: *mut libc::c_int,
    mut bucket_B: *mut libc::c_int,
    mut n: libc::c_int,
    mut m: libc::c_int,
    mut num_indexes: *mut libc::c_uchar,
    mut indexes: *mut libc::c_int,
) -> libc::c_int {
    let mut i = 0 as *mut libc::c_int;
    let mut j = 0 as *mut libc::c_int;
    let mut k = 0 as *mut libc::c_int;
    let mut orig = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut mod_0 = n / 8 as libc::c_int;
    mod_0 |= mod_0 >> 1 as libc::c_int;
    mod_0 |= mod_0 >> 2 as libc::c_int;
    mod_0 |= mod_0 >> 4 as libc::c_int;
    mod_0 |= mod_0 >> 8 as libc::c_int;
    mod_0 |= mod_0 >> 16 as libc::c_int;
    mod_0 >>= 1 as libc::c_int;
    *num_indexes = ((n - 1 as libc::c_int) / (mod_0 + 1 as libc::c_int))
        as libc::c_uchar;
    if (0 as libc::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as libc::c_int;
        while 0 as libc::c_int <= c1 {
            i = SA
                .offset(
                    *bucket_B
                        .offset(
                            (c1 << 8 as libc::c_int | c1 + 1 as libc::c_int) as isize,
                        ) as isize,
                );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as libc::c_int) as isize) as isize)
                .offset(-(1 as libc::c_int as isize));
            k = NULL as *mut libc::c_int;
            c2 = -(1 as libc::c_int);
            while i <= j {
                s = *j;
                if (0 as libc::c_int) < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {} else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1775 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[libc::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if (s + 1 as libc::c_int) < n
                        && *T.offset(s as isize) as libc::c_int
                            <= *T.offset((s + 1 as libc::c_int) as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1776 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[libc::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                        <= *T.offset(s as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1777 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[libc::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if s & mod_0 == 0 as libc::c_int {
                        *indexes
                            .offset(
                                (s / (mod_0 + 1 as libc::c_int) - 1 as libc::c_int) as isize,
                            ) = j.offset_from(SA) as libc::c_long as libc::c_int;
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    *j = !c0;
                    if (0 as libc::c_int) < s
                        && *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as libc::c_int <= c2 {
                            *bucket_B
                                .offset(
                                    (c1 << 8 as libc::c_int | c2) as isize,
                                ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(
                                *bucket_B.offset((c1 << 8 as libc::c_int | c2) as isize)
                                    as isize,
                            );
                    }
                    if k < j {} else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1788 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[libc::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {} else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const libc::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const libc::c_char,
                            1788 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[libc::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh92 = k;
                    k = k.offset(-1);
                    *fresh92 = s;
                } else if s != 0 as libc::c_int {
                    *j = !s;
                } else if *T.offset(s as isize) as libc::c_int == c1 {} else {
                    __assert_fail(
                        b"T[s] == c1\0" as *const u8 as *const libc::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const libc::c_char,
                        1794 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 104],
                            &[libc::c_char; 104],
                        >(
                            b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1 as libc::c_int) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    if (*T.offset((n - 2 as libc::c_int) as isize) as libc::c_int) < c2 {
        if n - 1 as libc::c_int & mod_0 == 0 as libc::c_int {
            *indexes
                .offset(
                    ((n - 1 as libc::c_int) / (mod_0 + 1 as libc::c_int)
                        - 1 as libc::c_int) as isize,
                ) = k.offset_from(SA) as libc::c_long as libc::c_int;
        }
        let fresh93 = k;
        k = k.offset(1);
        *fresh93 = !(*T.offset((n - 2 as libc::c_int) as isize) as libc::c_int);
    } else {
        let fresh94 = k;
        k = k.offset(1);
        *fresh94 = n - 1 as libc::c_int;
    }
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as libc::c_int) < s {
            if *T.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                >= *T.offset(s as isize) as libc::c_int
            {} else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1815 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if s & mod_0 == 0 as libc::c_int {
                *indexes
                    .offset(
                        (s / (mod_0 + 1 as libc::c_int) - 1 as libc::c_int) as isize,
                    ) = i.offset_from(SA) as libc::c_long as libc::c_int;
            }
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            *i = c0;
            if c0 != c2 {
                *bucket_A
                    .offset(
                        c2 as isize,
                    ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {} else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const libc::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const libc::c_char,
                    1825 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (0 as libc::c_int) < s
                && (*T.offset((s - 1 as libc::c_int) as isize) as libc::c_int) < c0
            {
                if s & mod_0 == 0 as libc::c_int {
                    *indexes
                        .offset(
                            (s / (mod_0 + 1 as libc::c_int) - 1 as libc::c_int) as isize,
                        ) = k.offset_from(SA) as libc::c_long as libc::c_int;
                }
                let fresh95 = k;
                k = k.offset(1);
                *fresh95 = !(*T.offset((s - 1 as libc::c_int) as isize) as libc::c_int);
            } else {
                let fresh96 = k;
                k = k.offset(1);
                *fresh96 = s;
            }
        } else if s != 0 as libc::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
    }
    return orig.offset_from(SA) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn divsufsort(
    mut T: *const libc::c_uchar,
    mut SA: *mut libc::c_int,
    mut n: libc::c_int,
    mut openMP: libc::c_int,
) -> libc::c_int {
    let mut bucket_A = 0 as *mut libc::c_int;
    let mut bucket_B = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut err = 0 as libc::c_int;
    if T.is_null() || SA.is_null() || n < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if n == 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if n == 1 as libc::c_int {
                *SA.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                return 0 as libc::c_int;
            } else {
                if n == 2 as libc::c_int {
                    m = ((*T.offset(0 as libc::c_int as isize) as libc::c_int)
                        < *T.offset(1 as libc::c_int as isize) as libc::c_int)
                        as libc::c_int;
                    *SA.offset((m ^ 1 as libc::c_int) as isize) = 0 as libc::c_int;
                    *SA.offset(m as isize) = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
        }
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, SA, bucket_A, bucket_B, n, openMP);
        construct_SA(T, SA, bucket_A, bucket_B, n, m);
    } else {
        err = -(2 as libc::c_int);
    }
    free(bucket_B as *mut libc::c_void);
    free(bucket_A as *mut libc::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn divbwt(
    mut T: *const libc::c_uchar,
    mut U: *mut libc::c_uchar,
    mut A: *mut libc::c_int,
    mut n: libc::c_int,
    mut num_indexes: *mut libc::c_uchar,
    mut indexes: *mut libc::c_int,
    mut openMP: libc::c_int,
) -> libc::c_int {
    let mut B = 0 as *mut libc::c_int;
    let mut bucket_A = 0 as *mut libc::c_int;
    let mut bucket_B = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut pidx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if T.is_null() || U.is_null() || n < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if n <= 1 as libc::c_int {
            if n == 1 as libc::c_int {
                *U
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *T.offset(0 as libc::c_int as isize);
            }
            return n;
        }
    }
    B = A;
    if B.is_null() {
        B = malloc(
            ((n + 1 as libc::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if !B.is_null() && !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, B, bucket_A, bucket_B, n, openMP);
        if num_indexes.is_null() || indexes.is_null() {
            pidx = construct_BWT(T, B, bucket_A, bucket_B, n, m);
        } else {
            pidx = construct_BWT_indexes(
                T,
                B,
                bucket_A,
                bucket_B,
                n,
                m,
                num_indexes,
                indexes,
            );
        }
        *U
            .offset(
                0 as libc::c_int as isize,
            ) = *T.offset((n - 1 as libc::c_int) as isize);
        i = 0 as libc::c_int;
        while i < pidx {
            *U
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = *B.offset(i as isize) as libc::c_uchar;
            i += 1;
        }
        i += 1 as libc::c_int;
        while i < n {
            *U.offset(i as isize) = *B.offset(i as isize) as libc::c_uchar;
            i += 1;
        }
        pidx += 1 as libc::c_int;
    } else {
        pidx = -(2 as libc::c_int);
    }
    free(bucket_B as *mut libc::c_void);
    free(bucket_A as *mut libc::c_void);
    if A.is_null() {
        free(B as *mut libc::c_void);
    }
    return pidx;
}
