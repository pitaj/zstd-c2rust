use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
}
pub type trbudget_t = _trbudget_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trbudget_t {
    pub chance: std::ffi::c_int,
    pub remain: std::ffi::c_int,
    pub incval: std::ffi::c_int,
    pub count: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub a: *const std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: *mut std::ffi::c_int,
    pub d: std::ffi::c_int,
    pub e: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: *mut std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: std::ffi::c_int,
    pub d: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: *mut std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: *mut std::ffi::c_int,
    pub d: std::ffi::c_int,
}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const ALPHABET_SIZE: std::ffi::c_int = 256 as std::ffi::c_int;
pub const BUCKET_A_SIZE: std::ffi::c_int = 256 as std::ffi::c_int;
pub const BUCKET_B_SIZE: std::ffi::c_int = ALPHABET_SIZE * ALPHABET_SIZE;
pub const SS_INSERTIONSORT_THRESHOLD: std::ffi::c_int = 8 as std::ffi::c_int;
pub const SS_BLOCKSIZE: std::ffi::c_int = 1024 as std::ffi::c_int;
pub const TR_INSERTIONSORT_THRESHOLD: std::ffi::c_int = 8 as std::ffi::c_int;
static mut lg_table: [std::ffi::c_int; 256] = [
    -(1 as std::ffi::c_int),
    0 as std::ffi::c_int,
    1 as std::ffi::c_int,
    1 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
];
#[inline]
unsafe extern "C" fn ss_ilg(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return if n & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + lg_table[(n >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    } else {
        0 as std::ffi::c_int
            + lg_table[(n >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    };
}
static mut sqq_table: [std::ffi::c_int; 256] = [
    0 as std::ffi::c_int,
    16 as std::ffi::c_int,
    22 as std::ffi::c_int,
    27 as std::ffi::c_int,
    32 as std::ffi::c_int,
    35 as std::ffi::c_int,
    39 as std::ffi::c_int,
    42 as std::ffi::c_int,
    45 as std::ffi::c_int,
    48 as std::ffi::c_int,
    50 as std::ffi::c_int,
    53 as std::ffi::c_int,
    55 as std::ffi::c_int,
    57 as std::ffi::c_int,
    59 as std::ffi::c_int,
    61 as std::ffi::c_int,
    64 as std::ffi::c_int,
    65 as std::ffi::c_int,
    67 as std::ffi::c_int,
    69 as std::ffi::c_int,
    71 as std::ffi::c_int,
    73 as std::ffi::c_int,
    75 as std::ffi::c_int,
    76 as std::ffi::c_int,
    78 as std::ffi::c_int,
    80 as std::ffi::c_int,
    81 as std::ffi::c_int,
    83 as std::ffi::c_int,
    84 as std::ffi::c_int,
    86 as std::ffi::c_int,
    87 as std::ffi::c_int,
    89 as std::ffi::c_int,
    90 as std::ffi::c_int,
    91 as std::ffi::c_int,
    93 as std::ffi::c_int,
    94 as std::ffi::c_int,
    96 as std::ffi::c_int,
    97 as std::ffi::c_int,
    98 as std::ffi::c_int,
    99 as std::ffi::c_int,
    101 as std::ffi::c_int,
    102 as std::ffi::c_int,
    103 as std::ffi::c_int,
    104 as std::ffi::c_int,
    106 as std::ffi::c_int,
    107 as std::ffi::c_int,
    108 as std::ffi::c_int,
    109 as std::ffi::c_int,
    110 as std::ffi::c_int,
    112 as std::ffi::c_int,
    113 as std::ffi::c_int,
    114 as std::ffi::c_int,
    115 as std::ffi::c_int,
    116 as std::ffi::c_int,
    117 as std::ffi::c_int,
    118 as std::ffi::c_int,
    119 as std::ffi::c_int,
    120 as std::ffi::c_int,
    121 as std::ffi::c_int,
    122 as std::ffi::c_int,
    123 as std::ffi::c_int,
    124 as std::ffi::c_int,
    125 as std::ffi::c_int,
    126 as std::ffi::c_int,
    128 as std::ffi::c_int,
    128 as std::ffi::c_int,
    129 as std::ffi::c_int,
    130 as std::ffi::c_int,
    131 as std::ffi::c_int,
    132 as std::ffi::c_int,
    133 as std::ffi::c_int,
    134 as std::ffi::c_int,
    135 as std::ffi::c_int,
    136 as std::ffi::c_int,
    137 as std::ffi::c_int,
    138 as std::ffi::c_int,
    139 as std::ffi::c_int,
    140 as std::ffi::c_int,
    141 as std::ffi::c_int,
    142 as std::ffi::c_int,
    143 as std::ffi::c_int,
    144 as std::ffi::c_int,
    144 as std::ffi::c_int,
    145 as std::ffi::c_int,
    146 as std::ffi::c_int,
    147 as std::ffi::c_int,
    148 as std::ffi::c_int,
    149 as std::ffi::c_int,
    150 as std::ffi::c_int,
    150 as std::ffi::c_int,
    151 as std::ffi::c_int,
    152 as std::ffi::c_int,
    153 as std::ffi::c_int,
    154 as std::ffi::c_int,
    155 as std::ffi::c_int,
    155 as std::ffi::c_int,
    156 as std::ffi::c_int,
    157 as std::ffi::c_int,
    158 as std::ffi::c_int,
    159 as std::ffi::c_int,
    160 as std::ffi::c_int,
    160 as std::ffi::c_int,
    161 as std::ffi::c_int,
    162 as std::ffi::c_int,
    163 as std::ffi::c_int,
    163 as std::ffi::c_int,
    164 as std::ffi::c_int,
    165 as std::ffi::c_int,
    166 as std::ffi::c_int,
    167 as std::ffi::c_int,
    167 as std::ffi::c_int,
    168 as std::ffi::c_int,
    169 as std::ffi::c_int,
    170 as std::ffi::c_int,
    170 as std::ffi::c_int,
    171 as std::ffi::c_int,
    172 as std::ffi::c_int,
    173 as std::ffi::c_int,
    173 as std::ffi::c_int,
    174 as std::ffi::c_int,
    175 as std::ffi::c_int,
    176 as std::ffi::c_int,
    176 as std::ffi::c_int,
    177 as std::ffi::c_int,
    178 as std::ffi::c_int,
    178 as std::ffi::c_int,
    179 as std::ffi::c_int,
    180 as std::ffi::c_int,
    181 as std::ffi::c_int,
    181 as std::ffi::c_int,
    182 as std::ffi::c_int,
    183 as std::ffi::c_int,
    183 as std::ffi::c_int,
    184 as std::ffi::c_int,
    185 as std::ffi::c_int,
    185 as std::ffi::c_int,
    186 as std::ffi::c_int,
    187 as std::ffi::c_int,
    187 as std::ffi::c_int,
    188 as std::ffi::c_int,
    189 as std::ffi::c_int,
    189 as std::ffi::c_int,
    190 as std::ffi::c_int,
    191 as std::ffi::c_int,
    192 as std::ffi::c_int,
    192 as std::ffi::c_int,
    193 as std::ffi::c_int,
    193 as std::ffi::c_int,
    194 as std::ffi::c_int,
    195 as std::ffi::c_int,
    195 as std::ffi::c_int,
    196 as std::ffi::c_int,
    197 as std::ffi::c_int,
    197 as std::ffi::c_int,
    198 as std::ffi::c_int,
    199 as std::ffi::c_int,
    199 as std::ffi::c_int,
    200 as std::ffi::c_int,
    201 as std::ffi::c_int,
    201 as std::ffi::c_int,
    202 as std::ffi::c_int,
    203 as std::ffi::c_int,
    203 as std::ffi::c_int,
    204 as std::ffi::c_int,
    204 as std::ffi::c_int,
    205 as std::ffi::c_int,
    206 as std::ffi::c_int,
    206 as std::ffi::c_int,
    207 as std::ffi::c_int,
    208 as std::ffi::c_int,
    208 as std::ffi::c_int,
    209 as std::ffi::c_int,
    209 as std::ffi::c_int,
    210 as std::ffi::c_int,
    211 as std::ffi::c_int,
    211 as std::ffi::c_int,
    212 as std::ffi::c_int,
    212 as std::ffi::c_int,
    213 as std::ffi::c_int,
    214 as std::ffi::c_int,
    214 as std::ffi::c_int,
    215 as std::ffi::c_int,
    215 as std::ffi::c_int,
    216 as std::ffi::c_int,
    217 as std::ffi::c_int,
    217 as std::ffi::c_int,
    218 as std::ffi::c_int,
    218 as std::ffi::c_int,
    219 as std::ffi::c_int,
    219 as std::ffi::c_int,
    220 as std::ffi::c_int,
    221 as std::ffi::c_int,
    221 as std::ffi::c_int,
    222 as std::ffi::c_int,
    222 as std::ffi::c_int,
    223 as std::ffi::c_int,
    224 as std::ffi::c_int,
    224 as std::ffi::c_int,
    225 as std::ffi::c_int,
    225 as std::ffi::c_int,
    226 as std::ffi::c_int,
    226 as std::ffi::c_int,
    227 as std::ffi::c_int,
    227 as std::ffi::c_int,
    228 as std::ffi::c_int,
    229 as std::ffi::c_int,
    229 as std::ffi::c_int,
    230 as std::ffi::c_int,
    230 as std::ffi::c_int,
    231 as std::ffi::c_int,
    231 as std::ffi::c_int,
    232 as std::ffi::c_int,
    232 as std::ffi::c_int,
    233 as std::ffi::c_int,
    234 as std::ffi::c_int,
    234 as std::ffi::c_int,
    235 as std::ffi::c_int,
    235 as std::ffi::c_int,
    236 as std::ffi::c_int,
    236 as std::ffi::c_int,
    237 as std::ffi::c_int,
    237 as std::ffi::c_int,
    238 as std::ffi::c_int,
    238 as std::ffi::c_int,
    239 as std::ffi::c_int,
    240 as std::ffi::c_int,
    240 as std::ffi::c_int,
    241 as std::ffi::c_int,
    241 as std::ffi::c_int,
    242 as std::ffi::c_int,
    242 as std::ffi::c_int,
    243 as std::ffi::c_int,
    243 as std::ffi::c_int,
    244 as std::ffi::c_int,
    244 as std::ffi::c_int,
    245 as std::ffi::c_int,
    245 as std::ffi::c_int,
    246 as std::ffi::c_int,
    246 as std::ffi::c_int,
    247 as std::ffi::c_int,
    247 as std::ffi::c_int,
    248 as std::ffi::c_int,
    248 as std::ffi::c_int,
    249 as std::ffi::c_int,
    249 as std::ffi::c_int,
    250 as std::ffi::c_int,
    250 as std::ffi::c_int,
    251 as std::ffi::c_int,
    251 as std::ffi::c_int,
    252 as std::ffi::c_int,
    252 as std::ffi::c_int,
    253 as std::ffi::c_int,
    253 as std::ffi::c_int,
    254 as std::ffi::c_int,
    254 as std::ffi::c_int,
    255 as std::ffi::c_int,
];
#[inline]
unsafe extern "C" fn ss_isqrt(mut x: std::ffi::c_int) -> std::ffi::c_int {
    let mut y: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    if x >= SS_BLOCKSIZE * SS_BLOCKSIZE {
        return SS_BLOCKSIZE;
    }
    e = if x as std::ffi::c_uint & 0xffff0000 as std::ffi::c_uint != 0 {
        if x as std::ffi::c_uint & 0xff000000 as std::ffi::c_uint != 0 {
            24 as std::ffi::c_int
                + lg_table[(x >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int)
                    as usize]
        } else {
            16 as std::ffi::c_int
                + lg_table[(x >> 16 as std::ffi::c_int & 0xff as std::ffi::c_int)
                    as usize]
        }
    } else if x & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + lg_table[(x >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    } else {
        0 as std::ffi::c_int
            + lg_table[(x >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    };
    if e >= 16 as std::ffi::c_int {
        y = sqq_table[(x >> e - 6 as std::ffi::c_int - (e & 1 as std::ffi::c_int))
            as usize] << (e >> 1 as std::ffi::c_int) - 7 as std::ffi::c_int;
        if e >= 24 as std::ffi::c_int {
            y = y + 1 as std::ffi::c_int + x / y >> 1 as std::ffi::c_int;
        }
        y = y + 1 as std::ffi::c_int + x / y >> 1 as std::ffi::c_int;
    } else if e >= 8 as std::ffi::c_int {
        y = (sqq_table[(x >> e - 6 as std::ffi::c_int - (e & 1 as std::ffi::c_int))
            as usize] >> 7 as std::ffi::c_int - (e >> 1 as std::ffi::c_int))
            + 1 as std::ffi::c_int;
    } else {
        return sqq_table[x as usize] >> 4 as std::ffi::c_int
    }
    return if x < y * y { y - 1 as std::ffi::c_int } else { y };
}
#[inline]
unsafe extern "C" fn ss_compare(
    mut T: *const std::ffi::c_uchar,
    mut p1: *const std::ffi::c_int,
    mut p2: *const std::ffi::c_int,
    mut depth: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut U1 = 0 as *const std::ffi::c_uchar;
    let mut U2 = 0 as *const std::ffi::c_uchar;
    let mut U1n = 0 as *const std::ffi::c_uchar;
    let mut U2n = 0 as *const std::ffi::c_uchar;
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T
        .offset(*p1.offset(1 as std::ffi::c_int as isize) as isize)
        .offset(2 as std::ffi::c_int as isize);
    U2n = T
        .offset(*p2.offset(1 as std::ffi::c_int as isize) as isize)
        .offset(2 as std::ffi::c_int as isize);
    while U1 < U1n && U2 < U2n && *U1 as std::ffi::c_int == *U2 as std::ffi::c_int {
        U1 = U1.offset(1);
        U1;
        U2 = U2.offset(1);
        U2;
    }
    return if U1 < U1n {
        if U2 < U2n {
            *U1 as std::ffi::c_int - *U2 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        }
    } else if U2 < U2n {
        -(1 as std::ffi::c_int)
    } else {
        0 as std::ffi::c_int
    };
}
unsafe extern "C" fn ss_insertionsort(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    i = last.offset(-(2 as std::ffi::c_int as isize));
    while first <= i {
        t = *i;
        j = i.offset(1 as std::ffi::c_int as isize);
        loop {
            r = ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize), depth);
            if !((0 as std::ffi::c_int) < r) {
                break;
            }
            loop {
                *j.offset(-(1 as std::ffi::c_int as isize)) = *j;
                j = j.offset(1);
                if !(j < last && *j < 0 as std::ffi::c_int) {
                    break;
                }
            }
            if last <= j {
                break;
            }
        }
        if r == 0 as std::ffi::c_int {
            *j = !*j;
        }
        *j.offset(-(1 as std::ffi::c_int as isize)) = t;
        i = i.offset(-1);
        i;
    }
}
#[inline]
unsafe extern "C" fn ss_fixdown(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut i: std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut c: std::ffi::c_int = 0;
    let mut d: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *Td.offset(*PA.offset(v as isize) as isize) as std::ffi::c_int;
    loop {
        j = 2 as std::ffi::c_int * i + 1 as std::ffi::c_int;
        if !(j < size) {
            break;
        }
        let fresh0 = j;
        j = j + 1;
        k = fresh0;
        d = *Td.offset(*PA.offset(*SA.offset(k as isize) as isize) as isize)
            as std::ffi::c_int;
        e = *Td.offset(*PA.offset(*SA.offset(j as isize) as isize) as isize)
            as std::ffi::c_int;
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
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    m = size;
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        m -= 1;
        m;
        if (*Td
            .offset(
                *PA.offset(*SA.offset((m / 2 as std::ffi::c_int) as isize) as isize)
                    as isize,
            ) as std::ffi::c_int)
            < *Td.offset(*PA.offset(*SA.offset(m as isize) as isize) as isize)
                as std::ffi::c_int
        {
            let ref mut fresh1 = SWAP!(SA[m], SA[m / 2]);
            *fresh1 = SWAP!(SA[m], SA[m / 2]);
            let ref mut fresh2 = SWAP!(SA[m], SA[m / 2]);
            *fresh2 = SWAP!(SA[m], SA[m / 2]);
            let ref mut fresh3 = SWAP!(SA[m], SA[m / 2]);
            *fresh3 = SWAP!(SA[m], SA[m / 2]);
        }
    }
    i = m / 2 as std::ffi::c_int - 1 as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        ss_fixdown(Td, PA, SA, i, m);
        i -= 1;
        i;
    }
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        let ref mut fresh4 = SWAP!(SA[0], SA[m]);
        *fresh4 = SWAP!(SA[0], SA[m]);
        let ref mut fresh5 = SWAP!(SA[0], SA[m]);
        *fresh5 = SWAP!(SA[0], SA[m]);
        let ref mut fresh6 = SWAP!(SA[0], SA[m]);
        *fresh6 = SWAP!(SA[0], SA[m]);
        ss_fixdown(Td, PA, SA, 0 as std::ffi::c_int, m);
    }
    i = m - 1 as std::ffi::c_int;
    while (0 as std::ffi::c_int) < i {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0 as std::ffi::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ss_median3(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh7 = SWAP!(v1, v2);
        *fresh7 = SWAP!(v1, v2);
        let ref mut fresh8 = SWAP!(v1, v2);
        *fresh8 = SWAP!(v1, v2);
        let ref mut fresh9 = SWAP!(v1, v2);
        *fresh9 = SWAP!(v1, v2);
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
            > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
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
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
    mut v4: *mut std::ffi::c_int,
    mut v5: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh10 = SWAP!(v2, v3);
        *fresh10 = SWAP!(v2, v3);
        let ref mut fresh11 = SWAP!(v2, v3);
        *fresh11 = SWAP!(v2, v3);
        let ref mut fresh12 = SWAP!(v2, v3);
        *fresh12 = SWAP!(v2, v3);
    }
    if *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v5 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh13 = SWAP!(v4, v5);
        *fresh13 = SWAP!(v4, v5);
        let ref mut fresh14 = SWAP!(v4, v5);
        *fresh14 = SWAP!(v4, v5);
        let ref mut fresh15 = SWAP!(v4, v5);
        *fresh15 = SWAP!(v4, v5);
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh16 = SWAP!(v2, v4);
        *fresh16 = SWAP!(v2, v4);
        let ref mut fresh17 = SWAP!(v2, v4);
        *fresh17 = SWAP!(v2, v4);
        let ref mut fresh18 = SWAP!(v2, v4);
        *fresh18 = SWAP!(v2, v4);
        let ref mut fresh19 = SWAP!(v3, v5);
        *fresh19 = SWAP!(v3, v5);
        let ref mut fresh20 = SWAP!(v3, v5);
        *fresh20 = SWAP!(v3, v5);
        let ref mut fresh21 = SWAP!(v3, v5);
        *fresh21 = SWAP!(v3, v5);
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh22 = SWAP!(v1, v3);
        *fresh22 = SWAP!(v1, v3);
        let ref mut fresh23 = SWAP!(v1, v3);
        *fresh23 = SWAP!(v1, v3);
        let ref mut fresh24 = SWAP!(v1, v3);
        *fresh24 = SWAP!(v1, v3);
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        let ref mut fresh25 = SWAP!(v1, v4);
        *fresh25 = SWAP!(v1, v4);
        let ref mut fresh26 = SWAP!(v1, v4);
        *fresh26 = SWAP!(v1, v4);
        let ref mut fresh27 = SWAP!(v1, v4);
        *fresh27 = SWAP!(v1, v4);
        let ref mut fresh28 = SWAP!(v3, v5);
        *fresh28 = SWAP!(v3, v5);
        let ref mut fresh29 = SWAP!(v3, v5);
        *fresh29 = SWAP!(v3, v5);
        let ref mut fresh30 = SWAP!(v3, v5);
        *fresh30 = SWAP!(v3, v5);
    }
    if *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        return v4;
    }
    return v3;
}
#[inline]
unsafe extern "C" fn ss_pivot(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    t = last.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    middle = first.offset((t / 2 as std::ffi::c_int) as isize);
    if t <= 512 as std::ffi::c_int {
        if t <= 32 as std::ffi::c_int {
            return ss_median3(
                Td,
                PA,
                first,
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)),
            )
        } else {
            t >>= 2 as std::ffi::c_int;
            return ss_median5(
                Td,
                PA,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)).offset(-(t as isize)),
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        }
    }
    t >>= 3 as std::ffi::c_int;
    first = ss_median3(
        Td,
        PA,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as std::ffi::c_int) as isize),
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
            .offset(-(1 as std::ffi::c_int as isize))
            .offset(-((t << 1 as std::ffi::c_int) as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)).offset(-(t as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)),
    );
    return ss_median3(Td, PA, first, middle, last);
}
#[inline]
unsafe extern "C" fn ss_partition(
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    a = first.offset(-(1 as std::ffi::c_int as isize));
    b = last;
    loop {
        loop {
            a = a.offset(1);
            if !(a < b
                && *PA.offset(*a as isize) + depth
                    >= *PA.offset((*a + 1 as std::ffi::c_int) as isize)
                        + 1 as std::ffi::c_int)
            {
                break;
            }
            *a = !*a;
        }
        loop {
            b = b.offset(-1);
            if !(a < b
                && *PA.offset(*b as isize) + depth
                    < *PA.offset((*b + 1 as std::ffi::c_int) as isize)
                        + 1 as std::ffi::c_int)
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
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_0; 16] = [C2RustUnnamed_0 {
        a: 0 as *mut std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0,
        d: 0,
    }; 16];
    let mut Td = 0 as *const std::ffi::c_uchar;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut f = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut limit: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    ssize = 0 as std::ffi::c_int;
    limit = ss_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
    loop {
        if last.offset_from(first) as std::ffi::c_long
            <= SS_INSERTIONSORT_THRESHOLD as std::ffi::c_long
        {
            if (1 as std::ffi::c_int as std::ffi::c_long)
                < last.offset_from(first) as std::ffi::c_long
            {
                ss_insertionsort(T, PA, first, last, depth);
            }
            if 0 as std::ffi::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const std::ffi::c_char,
                    418 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[std::ffi::c_char; 74],
                    >(
                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_15386: {
                if 0 as std::ffi::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const std::ffi::c_char,
                        418 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 74],
                            &[std::ffi::c_char; 74],
                        >(
                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if STACK_POP!(first, last, depth, limit) != 0 {
                return;
            }
        } else {
            Td = T.offset(depth as isize);
            let fresh31 = limit;
            limit = limit - 1;
            if fresh31 == 0 as std::ffi::c_int {
                ss_heapsort(
                    Td,
                    PA,
                    first,
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                );
            }
            if limit < 0 as std::ffi::c_int {
                a = first.offset(1 as std::ffi::c_int as isize);
                v = *Td.offset(*PA.offset(*first as isize) as isize) as std::ffi::c_int;
                while a < last {
                    x = *Td.offset(*PA.offset(*a as isize) as isize) as std::ffi::c_int;
                    if x != v {
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < a.offset_from(first) as std::ffi::c_long
                        {
                            break;
                        }
                        v = x;
                        first = a;
                    }
                    a = a.offset(1);
                    a;
                }
                if (*Td
                    .offset(
                        (*PA.offset(*first as isize) - 1 as std::ffi::c_int) as isize,
                    ) as std::ffi::c_int) < v
                {
                    first = ss_partition(PA, first, a, depth);
                }
                if a.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(a) as std::ffi::c_long
                {
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < a.offset_from(first) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                437 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_14736: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    437 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        last = a;
                        depth += 1 as std::ffi::c_int;
                        limit = ss_ilg(
                            a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                        );
                    } else {
                        first = a;
                        limit = -(1 as std::ffi::c_int);
                    }
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(a) as std::ffi::c_long
                {
                    if ssize < 16 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            444 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 74],
                                &[std::ffi::c_char; 74],
                            >(
                                b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_14607: {
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                444 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = a;
                    stack[ssize as usize].c = depth + 1 as std::ffi::c_int;
                    let fresh32 = ssize;
                    ssize = ssize + 1;
                    stack[fresh32 as usize]
                        .d = ss_ilg(
                        a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    );
                    first = a;
                    limit = -(1 as std::ffi::c_int);
                } else {
                    last = a;
                    depth += 1 as std::ffi::c_int;
                    limit = ss_ilg(
                        a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    );
                }
            } else {
                a = ss_pivot(Td, PA, first, last);
                v = *Td.offset(*PA.offset(*a as isize) as isize) as std::ffi::c_int;
                let ref mut fresh33 = SWAP!(* first, * a);
                *fresh33 = SWAP!(* first, * a);
                let ref mut fresh34 = SWAP!(* first, * a);
                *fresh34 = SWAP!(* first, * a);
                let ref mut fresh35 = SWAP!(* first, * a);
                *fresh35 = SWAP!(* first, * a);
                b = first;
                loop {
                    b = b.offset(1);
                    if !(b < last
                        && {
                            x = *Td.offset(*PA.offset(*b as isize) as isize)
                                as std::ffi::c_int;
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
                                    as std::ffi::c_int;
                                x <= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            let ref mut fresh36 = SWAP!(* b, * a);
                            *fresh36 = SWAP!(* b, * a);
                            let ref mut fresh37 = SWAP!(* b, * a);
                            *fresh37 = SWAP!(* b, * a);
                            let ref mut fresh38 = SWAP!(* b, * a);
                            *fresh38 = SWAP!(* b, * a);
                            a = a.offset(1);
                            a;
                        }
                    }
                }
                c = last;
                loop {
                    c = c.offset(-1);
                    if !(b < c
                        && {
                            x = *Td.offset(*PA.offset(*c as isize) as isize)
                                as std::ffi::c_int;
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
                                    as std::ffi::c_int;
                                x >= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            let ref mut fresh39 = SWAP!(* c, * d);
                            *fresh39 = SWAP!(* c, * d);
                            let ref mut fresh40 = SWAP!(* c, * d);
                            *fresh40 = SWAP!(* c, * d);
                            let ref mut fresh41 = SWAP!(* c, * d);
                            *fresh41 = SWAP!(* c, * d);
                            d = d.offset(-1);
                            d;
                        }
                    }
                }
                while b < c {
                    let ref mut fresh42 = SWAP!(* b, * c);
                    *fresh42 = SWAP!(* b, * c);
                    let ref mut fresh43 = SWAP!(* b, * c);
                    *fresh43 = SWAP!(* b, * c);
                    let ref mut fresh44 = SWAP!(* b, * c);
                    *fresh44 = SWAP!(* b, * c);
                    loop {
                        b = b.offset(1);
                        if !(b < c
                            && {
                                x = *Td.offset(*PA.offset(*b as isize) as isize)
                                    as std::ffi::c_int;
                                x <= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            let ref mut fresh45 = SWAP!(* b, * a);
                            *fresh45 = SWAP!(* b, * a);
                            let ref mut fresh46 = SWAP!(* b, * a);
                            *fresh46 = SWAP!(* b, * a);
                            let ref mut fresh47 = SWAP!(* b, * a);
                            *fresh47 = SWAP!(* b, * a);
                            a = a.offset(1);
                            a;
                        }
                    }
                    loop {
                        c = c.offset(-1);
                        if !(b < c
                            && {
                                x = *Td.offset(*PA.offset(*c as isize) as isize)
                                    as std::ffi::c_int;
                                x >= v
                            })
                        {
                            break;
                        }
                        if x == v {
                            let ref mut fresh48 = SWAP!(* c, * d);
                            *fresh48 = SWAP!(* c, * d);
                            let ref mut fresh49 = SWAP!(* c, * d);
                            *fresh49 = SWAP!(* c, * d);
                            let ref mut fresh50 = SWAP!(* c, * d);
                            *fresh50 = SWAP!(* c, * d);
                            d = d.offset(-1);
                            d;
                        }
                    }
                }
                if a <= d {
                    c = b.offset(-(1 as std::ffi::c_int as isize));
                    s = a.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
                    t = b.offset_from(a) as std::ffi::c_long as std::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = first;
                    f = b.offset(-(s as isize));
                    while (0 as std::ffi::c_int) < s {
                        let ref mut fresh51 = SWAP!(* e, * f);
                        *fresh51 = SWAP!(* e, * f);
                        let ref mut fresh52 = SWAP!(* e, * f);
                        *fresh52 = SWAP!(* e, * f);
                        let ref mut fresh53 = SWAP!(* e, * f);
                        *fresh53 = SWAP!(* e, * f);
                        s -= 1;
                        s;
                        e = e.offset(1);
                        e;
                        f = f.offset(1);
                        f;
                    }
                    s = d.offset_from(c) as std::ffi::c_long as std::ffi::c_int;
                    t = (last.offset_from(d) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = b;
                    f = last.offset(-(s as isize));
                    while (0 as std::ffi::c_int) < s {
                        let ref mut fresh54 = SWAP!(* e, * f);
                        *fresh54 = SWAP!(* e, * f);
                        let ref mut fresh55 = SWAP!(* e, * f);
                        *fresh55 = SWAP!(* e, * f);
                        let ref mut fresh56 = SWAP!(* e, * f);
                        *fresh56 = SWAP!(* e, * f);
                        s -= 1;
                        s;
                        e = e.offset(1);
                        e;
                        f = f.offset(1);
                        f;
                    }
                    a = first.offset(b.offset_from(a) as std::ffi::c_long as isize);
                    c = last.offset(-(d.offset_from(c) as std::ffi::c_long as isize));
                    b = if v
                        <= *Td
                            .offset(
                                (*PA.offset(*a as isize) - 1 as std::ffi::c_int) as isize,
                            ) as std::ffi::c_int
                    {
                        a
                    } else {
                        ss_partition(PA, a, c, depth)
                    };
                    if a.offset_from(first) as std::ffi::c_long
                        <= last.offset_from(c) as std::ffi::c_long
                    {
                        if last.offset_from(c) as std::ffi::c_long
                            <= c.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    494 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13070: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        494 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1 as std::ffi::c_int;
                            let fresh57 = ssize;
                            ssize = ssize + 1;
                            stack[fresh57 as usize]
                                .d = ss_ilg(
                                c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                            );
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    495 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_12976: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        495 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            last = a;
                        } else if a.offset_from(first) as std::ffi::c_long
                            <= c.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    498 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_12872: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        498 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    499 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_12787: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        499 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1 as std::ffi::c_int;
                            let fresh58 = ssize;
                            ssize = ssize + 1;
                            stack[fresh58 as usize]
                                .d = ss_ilg(
                                c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                            );
                            last = a;
                        } else {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    502 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_12686: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        502 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    503 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_12601: {
                                if ssize < 16 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        503 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            first = b;
                            last = c;
                            depth += 1 as std::ffi::c_int;
                            limit = ss_ilg(
                                c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                            );
                        }
                    } else if a.offset_from(first) as std::ffi::c_long
                        <= c.offset_from(b) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                508 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12469: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    508 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1 as std::ffi::c_int;
                        let fresh59 = ssize;
                        ssize = ssize + 1;
                        stack[fresh59 as usize]
                            .d = ss_ilg(
                            c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                        );
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                509 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12375: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    509 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        first = c;
                    } else if last.offset_from(c) as std::ffi::c_long
                        <= c.offset_from(b) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                512 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12271: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    512 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                513 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12186: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    513 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1 as std::ffi::c_int;
                        let fresh60 = ssize;
                        ssize = ssize + 1;
                        stack[fresh60 as usize]
                            .d = ss_ilg(
                            c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                        );
                        first = c;
                    } else {
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                516 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12085: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    516 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if ssize < 16 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                517 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_11998: {
                            if ssize < 16 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    517 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        first = b;
                        last = c;
                        depth += 1 as std::ffi::c_int;
                        limit = ss_ilg(
                            c.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                        );
                    }
                } else {
                    limit += 1 as std::ffi::c_int;
                    if (*Td
                        .offset(
                            (*PA.offset(*first as isize) - 1 as std::ffi::c_int) as isize,
                        ) as std::ffi::c_int) < v
                    {
                        first = ss_partition(PA, first, last, depth);
                        limit = ss_ilg(
                            last.offset_from(first) as std::ffi::c_long
                                as std::ffi::c_int,
                        );
                    }
                    depth += 1 as std::ffi::c_int;
                }
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ss_blockswap(
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
) {
    let mut t: std::ffi::c_int = 0;
    while (0 as std::ffi::c_int) < n {
        t = *a;
        *a = *b;
        *b = t;
        n -= 1;
        n;
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
    }
}
#[inline]
unsafe extern "C" fn ss_rotate(
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut l: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    l = middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    r = last.offset_from(middle) as std::ffi::c_long as std::ffi::c_int;
    while (0 as std::ffi::c_int) < l && (0 as std::ffi::c_int) < r {
        if l == r {
            ss_blockswap(first, middle, l);
            break;
        } else if l < r {
            a = last.offset(-(1 as std::ffi::c_int as isize));
            b = middle.offset(-(1 as std::ffi::c_int as isize));
            t = *a;
            loop {
                let fresh61 = a;
                a = a.offset(-1);
                *fresh61 = *b;
                let fresh62 = b;
                b = b.offset(-1);
                *fresh62 = *a;
                if !(b < first) {
                    continue;
                }
                *a = t;
                last = a;
                r -= l + 1 as std::ffi::c_int;
                if r <= l {
                    break;
                }
                a = a.offset(-(1 as std::ffi::c_int as isize));
                b = middle.offset(-(1 as std::ffi::c_int as isize));
                t = *a;
            }
        } else {
            a = first;
            b = middle;
            t = *a;
            loop {
                let fresh63 = a;
                a = a.offset(1);
                *fresh63 = *b;
                let fresh64 = b;
                b = b.offset(1);
                *fresh64 = *a;
                if !(last <= b) {
                    continue;
                }
                *a = t;
                first = a.offset(1 as std::ffi::c_int as isize);
                l -= r + 1 as std::ffi::c_int;
                if l <= r {
                    break;
                }
                a = a.offset(1 as std::ffi::c_int as isize);
                b = middle;
                t = *a;
            }
        }
    }
}
unsafe extern "C" fn ss_inplacemerge(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut p = 0 as *const std::ffi::c_int;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut len: std::ffi::c_int = 0;
    let mut half: std::ffi::c_int = 0;
    let mut q: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    loop {
        if *last.offset(-(1 as std::ffi::c_int as isize)) < 0 as std::ffi::c_int {
            x = 1 as std::ffi::c_int;
            p = PA.offset(!*last.offset(-(1 as std::ffi::c_int as isize)) as isize);
        } else {
            x = 0 as std::ffi::c_int;
            p = PA.offset(*last.offset(-(1 as std::ffi::c_int as isize)) as isize);
        }
        a = first;
        len = middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
        half = len >> 1 as std::ffi::c_int;
        r = -(1 as std::ffi::c_int);
        while (0 as std::ffi::c_int) < len {
            b = a.offset(half as isize);
            q = ss_compare(
                T,
                PA.offset((if 0 as std::ffi::c_int <= *b { *b } else { !*b }) as isize),
                p,
                depth,
            );
            if q < 0 as std::ffi::c_int {
                a = b.offset(1 as std::ffi::c_int as isize);
                half -= len & 1 as std::ffi::c_int ^ 1 as std::ffi::c_int;
            } else {
                r = q;
            }
            len = half;
            half >>= 1 as std::ffi::c_int;
        }
        if a < middle {
            if r == 0 as std::ffi::c_int {
                *a = !*a;
            }
            ss_rotate(a, middle, last);
            last = last.offset(-(middle.offset_from(a) as std::ffi::c_long as isize));
            middle = a;
            if first == middle {
                break;
            }
        }
        last = last.offset(-1);
        last;
        if x != 0 as std::ffi::c_int {
            loop {
                last = last.offset(-1);
                if !(*last < 0 as std::ffi::c_int) {
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
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut bufend = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    bufend = buf
        .offset(middle.offset_from(first) as std::ffi::c_long as isize)
        .offset(-(1 as std::ffi::c_int as isize));
    ss_blockswap(
        buf,
        first,
        middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
    );
    a = first;
    t = *a;
    b = buf;
    c = middle;
    loop {
        r = ss_compare(T, PA.offset(*b as isize), PA.offset(*c as isize), depth);
        if r < 0 as std::ffi::c_int {
            loop {
                let fresh65 = a;
                a = a.offset(1);
                *fresh65 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh66 = b;
                b = b.offset(1);
                *fresh66 = *a;
                if !(*b < 0 as std::ffi::c_int) {
                    break;
                }
            }
        } else if r > 0 as std::ffi::c_int {
            loop {
                let fresh67 = a;
                a = a.offset(1);
                *fresh67 = *c;
                let fresh68 = c;
                c = c.offset(1);
                *fresh68 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh69 = a;
                        a = a.offset(1);
                        *fresh69 = *b;
                        let fresh70 = b;
                        b = b.offset(1);
                        *fresh70 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if !(*c < 0 as std::ffi::c_int) {
                    break;
                }
            }
        } else {
            *c = !*c;
            loop {
                let fresh71 = a;
                a = a.offset(1);
                *fresh71 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh72 = b;
                b = b.offset(1);
                *fresh72 = *a;
                if !(*b < 0 as std::ffi::c_int) {
                    break;
                }
            }
            loop {
                let fresh73 = a;
                a = a.offset(1);
                *fresh73 = *c;
                let fresh74 = c;
                c = c.offset(1);
                *fresh74 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh75 = a;
                        a = a.offset(1);
                        *fresh75 = *b;
                        let fresh76 = b;
                        b = b.offset(1);
                        *fresh76 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if !(*c < 0 as std::ffi::c_int) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn ss_mergebackward(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut p1 = 0 as *const std::ffi::c_int;
    let mut p2 = 0 as *const std::ffi::c_int;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut bufend = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    bufend = buf
        .offset(last.offset_from(middle) as std::ffi::c_long as isize)
        .offset(-(1 as std::ffi::c_int as isize));
    ss_blockswap(
        buf,
        middle,
        last.offset_from(middle) as std::ffi::c_long as std::ffi::c_int,
    );
    x = 0 as std::ffi::c_int;
    if *bufend < 0 as std::ffi::c_int {
        p1 = PA.offset(!*bufend as isize);
        x |= 1 as std::ffi::c_int;
    } else {
        p1 = PA.offset(*bufend as isize);
    }
    if *middle.offset(-(1 as std::ffi::c_int as isize)) < 0 as std::ffi::c_int {
        p2 = PA.offset(!*middle.offset(-(1 as std::ffi::c_int as isize)) as isize);
        x |= 2 as std::ffi::c_int;
    } else {
        p2 = PA.offset(*middle.offset(-(1 as std::ffi::c_int as isize)) as isize);
    }
    a = last.offset(-(1 as std::ffi::c_int as isize));
    t = *a;
    b = bufend;
    c = middle.offset(-(1 as std::ffi::c_int as isize));
    loop {
        r = ss_compare(T, p1, p2, depth);
        if (0 as std::ffi::c_int) < r {
            if x & 1 as std::ffi::c_int != 0 {
                loop {
                    let fresh77 = a;
                    a = a.offset(-1);
                    *fresh77 = *b;
                    let fresh78 = b;
                    b = b.offset(-1);
                    *fresh78 = *a;
                    if !(*b < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 1 as std::ffi::c_int;
            }
            let fresh79 = a;
            a = a.offset(-1);
            *fresh79 = *b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh80 = b;
                b = b.offset(-1);
                *fresh80 = *a;
                if *b < 0 as std::ffi::c_int {
                    p1 = PA.offset(!*b as isize);
                    x |= 1 as std::ffi::c_int;
                } else {
                    p1 = PA.offset(*b as isize);
                }
            }
        } else if r < 0 as std::ffi::c_int {
            if x & 2 as std::ffi::c_int != 0 {
                loop {
                    let fresh81 = a;
                    a = a.offset(-1);
                    *fresh81 = *c;
                    let fresh82 = c;
                    c = c.offset(-1);
                    *fresh82 = *a;
                    if !(*c < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 2 as std::ffi::c_int;
            }
            let fresh83 = a;
            a = a.offset(-1);
            *fresh83 = *c;
            let fresh84 = c;
            c = c.offset(-1);
            *fresh84 = *a;
            if c < first {
                while buf < b {
                    let fresh85 = a;
                    a = a.offset(-1);
                    *fresh85 = *b;
                    let fresh86 = b;
                    b = b.offset(-1);
                    *fresh86 = *a;
                }
                *a = *b;
                *b = t;
                break;
            } else if *c < 0 as std::ffi::c_int {
                p2 = PA.offset(!*c as isize);
                x |= 2 as std::ffi::c_int;
            } else {
                p2 = PA.offset(*c as isize);
            }
        } else {
            if x & 1 as std::ffi::c_int != 0 {
                loop {
                    let fresh87 = a;
                    a = a.offset(-1);
                    *fresh87 = *b;
                    let fresh88 = b;
                    b = b.offset(-1);
                    *fresh88 = *a;
                    if !(*b < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 1 as std::ffi::c_int;
            }
            let fresh89 = a;
            a = a.offset(-1);
            *fresh89 = !*b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh90 = b;
                b = b.offset(-1);
                *fresh90 = *a;
                if x & 2 as std::ffi::c_int != 0 {
                    loop {
                        let fresh91 = a;
                        a = a.offset(-1);
                        *fresh91 = *c;
                        let fresh92 = c;
                        c = c.offset(-1);
                        *fresh92 = *a;
                        if !(*c < 0 as std::ffi::c_int) {
                            break;
                        }
                    }
                    x ^= 2 as std::ffi::c_int;
                }
                let fresh93 = a;
                a = a.offset(-1);
                *fresh93 = *c;
                let fresh94 = c;
                c = c.offset(-1);
                *fresh94 = *a;
                if c < first {
                    while buf < b {
                        let fresh95 = a;
                        a = a.offset(-1);
                        *fresh95 = *b;
                        let fresh96 = b;
                        b = b.offset(-1);
                        *fresh96 = *a;
                    }
                    *a = *b;
                    *b = t;
                    break;
                } else {
                    if *b < 0 as std::ffi::c_int {
                        p1 = PA.offset(!*b as isize);
                        x |= 1 as std::ffi::c_int;
                    } else {
                        p1 = PA.offset(*b as isize);
                    }
                    if *c < 0 as std::ffi::c_int {
                        p2 = PA.offset(!*c as isize);
                        x |= 2 as std::ffi::c_int;
                    } else {
                        p2 = PA.offset(*c as isize);
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn ss_swapmerge(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut bufsize: std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_1; 32] = [C2RustUnnamed_1 {
        a: 0 as *mut std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0 as *mut std::ffi::c_int,
        d: 0,
    }; 32];
    let mut l = 0 as *mut std::ffi::c_int;
    let mut r = 0 as *mut std::ffi::c_int;
    let mut lm = 0 as *mut std::ffi::c_int;
    let mut rm = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut len: std::ffi::c_int = 0;
    let mut half: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut check: std::ffi::c_int = 0;
    let mut next: std::ffi::c_int = 0;
    check = 0 as std::ffi::c_int;
    ssize = 0 as std::ffi::c_int;
    loop {
        if last.offset_from(middle) as std::ffi::c_long <= bufsize as std::ffi::c_long {
            if first < middle && middle < last {
                ss_mergebackward(T, PA, first, middle, last, buf, depth);
            }
            if MERGE_CHECK!(first, last, check) != 0 {
                let ref mut fresh97 = MERGE_CHECK!(first, last, check);
                *fresh97 = MERGE_CHECK!(first, last, check);
            }
            if MERGE_CHECK!(first, last, check) != 0 {
                let ref mut fresh98 = MERGE_CHECK!(first, last, check);
                *fresh98 = MERGE_CHECK!(first, last, check);
            }
            if 0 as std::ffi::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const std::ffi::c_char,
                    771 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[std::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_17274: {
                if 0 as std::ffi::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const std::ffi::c_char,
                        771 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if STACK_POP!(first, middle, last, check) != 0 {
                return;
            }
        } else if middle.offset_from(first) as std::ffi::c_long
            <= bufsize as std::ffi::c_long
        {
            if first < middle {
                ss_mergeforward(T, PA, first, middle, last, buf, depth);
            }
            if MERGE_CHECK!(first, last, check) != 0 {
                let ref mut fresh99 = MERGE_CHECK!(first, last, check);
                *fresh99 = MERGE_CHECK!(first, last, check);
            }
            if MERGE_CHECK!(first, last, check) != 0 {
                let ref mut fresh100 = MERGE_CHECK!(first, last, check);
                *fresh100 = MERGE_CHECK!(first, last, check);
            }
            if 0 as std::ffi::c_int <= ssize {} else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                        as *const u8 as *const std::ffi::c_char,
                    780 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[std::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_16633: {
                if 0 as std::ffi::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const std::ffi::c_char,
                        780 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if STACK_POP!(first, middle, last, check) != 0 {
                return;
            }
        } else {
            m = 0 as std::ffi::c_int;
            len = MIN!(middle - first, last - middle);
            half = len >> 1 as std::ffi::c_int;
            while (0 as std::ffi::c_int) < len {
                if ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as std::ffi::c_int
                                <= *middle.offset(m as isize).offset(half as isize)
                            {
                                *middle.offset(m as isize).offset(half as isize)
                            } else {
                                !*middle.offset(m as isize).offset(half as isize)
                            }) as isize,
                        ),
                    PA
                        .offset(
                            (if 0 as std::ffi::c_int
                                <= *middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as std::ffi::c_int as isize))
                            {
                                *middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as std::ffi::c_int as isize))
                            } else {
                                !*middle
                                    .offset(-(m as isize))
                                    .offset(-(half as isize))
                                    .offset(-(1 as std::ffi::c_int as isize))
                            }) as isize,
                        ),
                    depth,
                ) < 0 as std::ffi::c_int
                {
                    m += half + 1 as std::ffi::c_int;
                    half -= len & 1 as std::ffi::c_int ^ 1 as std::ffi::c_int;
                }
                len = half;
                half >>= 1 as std::ffi::c_int;
            }
            if (0 as std::ffi::c_int) < m {
                lm = middle.offset(-(m as isize));
                rm = middle.offset(m as isize);
                ss_blockswap(lm, middle, m);
                r = middle;
                l = r;
                next = 0 as std::ffi::c_int;
                if rm < last {
                    if *rm < 0 as std::ffi::c_int {
                        *rm = !*rm;
                        if first < lm {
                            loop {
                                l = l.offset(-1);
                                if !(*l < 0 as std::ffi::c_int) {
                                    break;
                                }
                            }
                            next |= 4 as std::ffi::c_int;
                        }
                        next |= 1 as std::ffi::c_int;
                    } else if first < lm {
                        while *r < 0 as std::ffi::c_int {
                            r = r.offset(1);
                            r;
                        }
                        next |= 2 as std::ffi::c_int;
                    }
                }
                if l.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(r) as std::ffi::c_long
                {
                    if ssize < 32 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            810 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_16232: {
                        if ssize < 32 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                810 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 92],
                                    &[std::ffi::c_char; 92],
                                >(
                                    b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    stack[ssize as usize].a = r;
                    stack[ssize as usize].b = rm;
                    stack[ssize as usize].c = last;
                    let fresh101 = ssize;
                    ssize = ssize + 1;
                    stack[fresh101 as usize]
                        .d = next & 3 as std::ffi::c_int | check & 4 as std::ffi::c_int;
                    middle = lm;
                    last = l;
                    check = check & 3 as std::ffi::c_int | next & 4 as std::ffi::c_int;
                } else {
                    if next & 2 as std::ffi::c_int != 0 && r == middle {
                        next ^= 6 as std::ffi::c_int;
                    }
                    if ssize < 32 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            814 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_16096: {
                        if ssize < 32 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                814 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 92],
                                    &[std::ffi::c_char; 92],
                                >(
                                    b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = lm;
                    stack[ssize as usize].c = l;
                    let fresh102 = ssize;
                    ssize = ssize + 1;
                    stack[fresh102 as usize]
                        .d = check & 3 as std::ffi::c_int | next & 4 as std::ffi::c_int;
                    first = r;
                    middle = rm;
                    check = next & 3 as std::ffi::c_int | check & 4 as std::ffi::c_int;
                }
            } else {
                if ss_compare(
                    T,
                    PA
                        .offset(
                            (if 0 as std::ffi::c_int
                                <= *middle.offset(-(1 as std::ffi::c_int as isize))
                            {
                                *middle.offset(-(1 as std::ffi::c_int as isize))
                            } else {
                                !*middle.offset(-(1 as std::ffi::c_int as isize))
                            }) as isize,
                        ),
                    PA.offset(*middle as isize),
                    depth,
                ) == 0 as std::ffi::c_int
                {
                    *middle = !*middle;
                }
                if MERGE_CHECK!(first, last, check) != 0 {
                    let ref mut fresh103 = MERGE_CHECK!(first, last, check);
                    *fresh103 = MERGE_CHECK!(first, last, check);
                }
                if MERGE_CHECK!(first, last, check) != 0 {
                    let ref mut fresh104 = MERGE_CHECK!(first, last, check);
                    *fresh104 = MERGE_CHECK!(first, last, check);
                }
                if 0 as std::ffi::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const std::ffi::c_char,
                        822 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_15739: {
                    if 0 as std::ffi::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            822 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if STACK_POP!(first, middle, last, check) != 0 {
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn sssort(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut bufsize: std::ffi::c_int,
    mut depth: std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut lastsuffix: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut curbuf = 0 as *mut std::ffi::c_int;
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut curbufsize: std::ffi::c_int = 0;
    let mut limit: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    if lastsuffix != 0 as std::ffi::c_int {
        first = first.offset(1);
        first;
    }
    if bufsize < SS_BLOCKSIZE
        && (bufsize as std::ffi::c_long) < last.offset_from(first) as std::ffi::c_long
        && {
            limit = ss_isqrt(
                last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
            );
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
        limit = 0 as std::ffi::c_int;
    }
    a = first;
    i = 0 as std::ffi::c_int;
    while (SS_BLOCKSIZE as std::ffi::c_long) < middle.offset_from(a) as std::ffi::c_long
    {
        ss_mintrosort(T, PA, a, a.offset(SS_BLOCKSIZE as isize), depth);
        curbufsize = last.offset_from(a.offset(SS_BLOCKSIZE as isize))
            as std::ffi::c_long as std::ffi::c_int;
        curbuf = a.offset(SS_BLOCKSIZE as isize);
        if curbufsize <= bufsize {
            curbufsize = bufsize;
            curbuf = buf;
        }
        b = a;
        k = SS_BLOCKSIZE;
        j = i;
        while j & 1 as std::ffi::c_int != 0 {
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
            k <<= 1 as std::ffi::c_int;
            j >>= 1 as std::ffi::c_int;
        }
        a = a.offset(SS_BLOCKSIZE as isize);
        i += 1;
        i;
    }
    ss_mintrosort(T, PA, a, middle, depth);
    k = SS_BLOCKSIZE;
    while i != 0 as std::ffi::c_int {
        if i & 1 as std::ffi::c_int != 0 {
            ss_swapmerge(T, PA, a.offset(-(k as isize)), a, middle, buf, bufsize, depth);
            a = a.offset(-(k as isize));
        }
        k <<= 1 as std::ffi::c_int;
        i >>= 1 as std::ffi::c_int;
    }
    if limit != 0 as std::ffi::c_int {
        ss_mintrosort(T, PA, middle, last, depth);
        ss_inplacemerge(T, PA, first, middle, last, depth);
    }
    if lastsuffix != 0 as std::ffi::c_int {
        let mut PAi: [std::ffi::c_int; 2] = [0; 2];
        PAi[0 as std::ffi::c_int
            as usize] = *PA
            .offset(*first.offset(-(1 as std::ffi::c_int as isize)) as isize);
        PAi[1 as std::ffi::c_int as usize] = n - 2 as std::ffi::c_int;
        a = first;
        i = *first.offset(-(1 as std::ffi::c_int as isize));
        while a < last
            && (*a < 0 as std::ffi::c_int
                || (0 as std::ffi::c_int)
                    < ss_compare(
                        T,
                        &mut *PAi.as_mut_ptr().offset(0 as std::ffi::c_int as isize),
                        PA.offset(*a as isize),
                        depth,
                    ))
        {
            *a.offset(-(1 as std::ffi::c_int as isize)) = *a;
            a = a.offset(1);
            a;
        }
        *a.offset(-(1 as std::ffi::c_int as isize)) = i;
    }
}
#[inline]
unsafe extern "C" fn tr_ilg(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return if n as std::ffi::c_uint & 0xffff0000 as std::ffi::c_uint != 0 {
        if n as std::ffi::c_uint & 0xff000000 as std::ffi::c_uint != 0 {
            24 as std::ffi::c_int
                + lg_table[(n >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int)
                    as usize]
        } else {
            16 as std::ffi::c_int
                + lg_table[(n >> 16 as std::ffi::c_int & 0xff as std::ffi::c_int)
                    as usize]
        }
    } else if n & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + lg_table[(n >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    } else {
        0 as std::ffi::c_int
            + lg_table[(n >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as usize]
    };
}
unsafe extern "C" fn tr_insertionsort(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    a = first.offset(1 as std::ffi::c_int as isize);
    while a < last {
        t = *a;
        b = a.offset(-(1 as std::ffi::c_int as isize));
        loop {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if !(0 as std::ffi::c_int > r) {
                break;
            }
            loop {
                *b.offset(1 as std::ffi::c_int as isize) = *b;
                b = b.offset(-1);
                if !(first <= b && *b < 0 as std::ffi::c_int) {
                    break;
                }
            }
            if b < first {
                break;
            }
        }
        if r == 0 as std::ffi::c_int {
            *b = !*b;
        }
        *b.offset(1 as std::ffi::c_int as isize) = t;
        a = a.offset(1);
        a;
    }
}
#[inline]
unsafe extern "C" fn tr_fixdown(
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut i: std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut c: std::ffi::c_int = 0;
    let mut d: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *ISAd.offset(v as isize);
    loop {
        j = 2 as std::ffi::c_int * i + 1 as std::ffi::c_int;
        if !(j < size) {
            break;
        }
        let fresh105 = j;
        j = j + 1;
        k = fresh105;
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
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    m = size;
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        m -= 1;
        m;
        if *ISAd.offset(*SA.offset((m / 2 as std::ffi::c_int) as isize) as isize)
            < *ISAd.offset(*SA.offset(m as isize) as isize)
        {
            let ref mut fresh106 = SWAP!(SA[m], SA[m / 2]);
            *fresh106 = SWAP!(SA[m], SA[m / 2]);
            let ref mut fresh107 = SWAP!(SA[m], SA[m / 2]);
            *fresh107 = SWAP!(SA[m], SA[m / 2]);
            let ref mut fresh108 = SWAP!(SA[m], SA[m / 2]);
            *fresh108 = SWAP!(SA[m], SA[m / 2]);
        }
    }
    i = m / 2 as std::ffi::c_int - 1 as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        tr_fixdown(ISAd, SA, i, m);
        i -= 1;
        i;
    }
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        let ref mut fresh109 = SWAP!(SA[0], SA[m]);
        *fresh109 = SWAP!(SA[0], SA[m]);
        let ref mut fresh110 = SWAP!(SA[0], SA[m]);
        *fresh110 = SWAP!(SA[0], SA[m]);
        let ref mut fresh111 = SWAP!(SA[0], SA[m]);
        *fresh111 = SWAP!(SA[0], SA[m]);
        tr_fixdown(ISAd, SA, 0 as std::ffi::c_int, m);
    }
    i = m - 1 as std::ffi::c_int;
    while (0 as std::ffi::c_int) < i {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0 as std::ffi::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn tr_median3(
    mut ISAd: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v2 as isize) {
        let ref mut fresh112 = SWAP!(v1, v2);
        *fresh112 = SWAP!(v1, v2);
        let ref mut fresh113 = SWAP!(v1, v2);
        *fresh113 = SWAP!(v1, v2);
        let ref mut fresh114 = SWAP!(v1, v2);
        *fresh114 = SWAP!(v1, v2);
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
    mut ISAd: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
    mut v4: *mut std::ffi::c_int,
    mut v5: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        let ref mut fresh115 = SWAP!(v2, v3);
        *fresh115 = SWAP!(v2, v3);
        let ref mut fresh116 = SWAP!(v2, v3);
        *fresh116 = SWAP!(v2, v3);
        let ref mut fresh117 = SWAP!(v2, v3);
        *fresh117 = SWAP!(v2, v3);
    }
    if *ISAd.offset(*v4 as isize) > *ISAd.offset(*v5 as isize) {
        let ref mut fresh118 = SWAP!(v4, v5);
        *fresh118 = SWAP!(v4, v5);
        let ref mut fresh119 = SWAP!(v4, v5);
        *fresh119 = SWAP!(v4, v5);
        let ref mut fresh120 = SWAP!(v4, v5);
        *fresh120 = SWAP!(v4, v5);
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v4 as isize) {
        let ref mut fresh121 = SWAP!(v2, v4);
        *fresh121 = SWAP!(v2, v4);
        let ref mut fresh122 = SWAP!(v2, v4);
        *fresh122 = SWAP!(v2, v4);
        let ref mut fresh123 = SWAP!(v2, v4);
        *fresh123 = SWAP!(v2, v4);
        let ref mut fresh124 = SWAP!(v3, v5);
        *fresh124 = SWAP!(v3, v5);
        let ref mut fresh125 = SWAP!(v3, v5);
        *fresh125 = SWAP!(v3, v5);
        let ref mut fresh126 = SWAP!(v3, v5);
        *fresh126 = SWAP!(v3, v5);
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
        let ref mut fresh127 = SWAP!(v1, v3);
        *fresh127 = SWAP!(v1, v3);
        let ref mut fresh128 = SWAP!(v1, v3);
        *fresh128 = SWAP!(v1, v3);
        let ref mut fresh129 = SWAP!(v1, v3);
        *fresh129 = SWAP!(v1, v3);
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v4 as isize) {
        let ref mut fresh130 = SWAP!(v1, v4);
        *fresh130 = SWAP!(v1, v4);
        let ref mut fresh131 = SWAP!(v1, v4);
        *fresh131 = SWAP!(v1, v4);
        let ref mut fresh132 = SWAP!(v1, v4);
        *fresh132 = SWAP!(v1, v4);
        let ref mut fresh133 = SWAP!(v3, v5);
        *fresh133 = SWAP!(v3, v5);
        let ref mut fresh134 = SWAP!(v3, v5);
        *fresh134 = SWAP!(v3, v5);
        let ref mut fresh135 = SWAP!(v3, v5);
        *fresh135 = SWAP!(v3, v5);
    }
    if *ISAd.offset(*v3 as isize) > *ISAd.offset(*v4 as isize) {
        return v4;
    }
    return v3;
}
#[inline]
unsafe extern "C" fn tr_pivot(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    t = last.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    middle = first.offset((t / 2 as std::ffi::c_int) as isize);
    if t <= 512 as std::ffi::c_int {
        if t <= 32 as std::ffi::c_int {
            return tr_median3(
                ISAd,
                first,
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)),
            )
        } else {
            t >>= 2 as std::ffi::c_int;
            return tr_median5(
                ISAd,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)).offset(-(t as isize)),
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        }
    }
    t >>= 3 as std::ffi::c_int;
    first = tr_median3(
        ISAd,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as std::ffi::c_int) as isize),
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
            .offset(-(1 as std::ffi::c_int as isize))
            .offset(-((t << 1 as std::ffi::c_int) as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)).offset(-(t as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)),
    );
    return tr_median3(ISAd, first, middle, last);
}
#[inline]
unsafe extern "C" fn trbudget_init(
    mut budget: *mut trbudget_t,
    mut chance: std::ffi::c_int,
    mut incval: std::ffi::c_int,
) {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
#[inline]
unsafe extern "C" fn trbudget_check(
    mut budget: *mut trbudget_t,
    mut size: std::ffi::c_int,
) -> std::ffi::c_int {
    if size <= (*budget).remain {
        (*budget).remain -= size;
        return 1 as std::ffi::c_int;
    }
    if (*budget).chance == 0 as std::ffi::c_int {
        (*budget).count += size;
        return 0 as std::ffi::c_int;
    }
    (*budget).remain += (*budget).incval - size;
    (*budget).chance -= 1 as std::ffi::c_int;
    return 1 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn tr_partition(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut pa: *mut *mut std::ffi::c_int,
    mut pb: *mut *mut std::ffi::c_int,
    mut v: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut f = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    b = middle.offset(-(1 as std::ffi::c_int as isize));
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
                let ref mut fresh136 = SWAP!(* b, * a);
                *fresh136 = SWAP!(* b, * a);
                let ref mut fresh137 = SWAP!(* b, * a);
                *fresh137 = SWAP!(* b, * a);
                let ref mut fresh138 = SWAP!(* b, * a);
                *fresh138 = SWAP!(* b, * a);
                a = a.offset(1);
                a;
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
                let ref mut fresh139 = SWAP!(* c, * d);
                *fresh139 = SWAP!(* c, * d);
                let ref mut fresh140 = SWAP!(* c, * d);
                *fresh140 = SWAP!(* c, * d);
                let ref mut fresh141 = SWAP!(* c, * d);
                *fresh141 = SWAP!(* c, * d);
                d = d.offset(-1);
                d;
            }
        }
    }
    while b < c {
        let ref mut fresh142 = SWAP!(* b, * c);
        *fresh142 = SWAP!(* b, * c);
        let ref mut fresh143 = SWAP!(* b, * c);
        *fresh143 = SWAP!(* b, * c);
        let ref mut fresh144 = SWAP!(* b, * c);
        *fresh144 = SWAP!(* b, * c);
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
                let ref mut fresh145 = SWAP!(* b, * a);
                *fresh145 = SWAP!(* b, * a);
                let ref mut fresh146 = SWAP!(* b, * a);
                *fresh146 = SWAP!(* b, * a);
                let ref mut fresh147 = SWAP!(* b, * a);
                *fresh147 = SWAP!(* b, * a);
                a = a.offset(1);
                a;
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
                let ref mut fresh148 = SWAP!(* c, * d);
                *fresh148 = SWAP!(* c, * d);
                let ref mut fresh149 = SWAP!(* c, * d);
                *fresh149 = SWAP!(* c, * d);
                let ref mut fresh150 = SWAP!(* c, * d);
                *fresh150 = SWAP!(* c, * d);
                d = d.offset(-1);
                d;
            }
        }
    }
    if a <= d {
        c = b.offset(-(1 as std::ffi::c_int as isize));
        s = a.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
        t = b.offset_from(a) as std::ffi::c_long as std::ffi::c_int;
        if s > t {
            s = t;
        }
        e = first;
        f = b.offset(-(s as isize));
        while (0 as std::ffi::c_int) < s {
            let ref mut fresh151 = SWAP!(* e, * f);
            *fresh151 = SWAP!(* e, * f);
            let ref mut fresh152 = SWAP!(* e, * f);
            *fresh152 = SWAP!(* e, * f);
            let ref mut fresh153 = SWAP!(* e, * f);
            *fresh153 = SWAP!(* e, * f);
            s -= 1;
            s;
            e = e.offset(1);
            e;
            f = f.offset(1);
            f;
        }
        s = d.offset_from(c) as std::ffi::c_long as std::ffi::c_int;
        t = (last.offset_from(d) as std::ffi::c_long
            - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
        if s > t {
            s = t;
        }
        e = b;
        f = last.offset(-(s as isize));
        while (0 as std::ffi::c_int) < s {
            let ref mut fresh154 = SWAP!(* e, * f);
            *fresh154 = SWAP!(* e, * f);
            let ref mut fresh155 = SWAP!(* e, * f);
            *fresh155 = SWAP!(* e, * f);
            let ref mut fresh156 = SWAP!(* e, * f);
            *fresh156 = SWAP!(* e, * f);
            s -= 1;
            s;
            e = e.offset(1);
            e;
            f = f.offset(1);
            f;
        }
        first = first.offset(b.offset_from(a) as std::ffi::c_long as isize);
        last = last.offset(-(d.offset_from(c) as std::ffi::c_long as isize));
    }
    *pa = first;
    *pb = last;
}
unsafe extern "C" fn tr_copy(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    v = (b.offset_from(SA) as std::ffi::c_long
        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
    c = first;
    d = a.offset(-(1 as std::ffi::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            *ISA
                .offset(
                    s as isize,
                ) = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        c = c.offset(1);
        c;
    }
    c = last.offset(-(1 as std::ffi::c_int as isize));
    e = d.offset(1 as std::ffi::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            *ISA
                .offset(
                    s as isize,
                ) = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        c = c.offset(-1);
        c;
    }
}
unsafe extern "C" fn tr_partialcopy(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut rank: std::ffi::c_int = 0;
    let mut lastrank: std::ffi::c_int = 0;
    let mut newrank = -(1 as std::ffi::c_int);
    v = (b.offset_from(SA) as std::ffi::c_long
        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
    lastrank = -(1 as std::ffi::c_int);
    c = first;
    d = a.offset(-(1 as std::ffi::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(1);
        c;
    }
    lastrank = -(1 as std::ffi::c_int);
    e = d;
    while first <= e {
        rank = *ISA.offset(*e as isize);
        if lastrank != rank {
            lastrank = rank;
            newrank = e.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        if newrank != rank {
            *ISA.offset(*e as isize) = newrank;
        }
        e = e.offset(-1);
        e;
    }
    lastrank = -(1 as std::ffi::c_int);
    c = last.offset(-(1 as std::ffi::c_int as isize));
    e = d.offset(1 as std::ffi::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(-1);
        c;
    }
}
unsafe extern "C" fn tr_introsort(
    mut ISA: *mut std::ffi::c_int,
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut budget: *mut trbudget_t,
) {
    let mut stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
        a: 0 as *const std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0 as *mut std::ffi::c_int,
        d: 0,
        e: 0,
    }; 64];
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    let mut incr = ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int;
    let mut limit: std::ffi::c_int = 0;
    let mut next: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut trlink = -(1 as std::ffi::c_int);
    ssize = 0 as std::ffi::c_int;
    limit = tr_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
    loop {
        if limit < 0 as std::ffi::c_int {
            if limit == -(1 as std::ffi::c_int) {
                tr_partition(
                    ISAd.offset(-(incr as isize)),
                    first,
                    first,
                    last,
                    &mut a,
                    &mut b,
                    (last.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int,
                );
                if a < last {
                    c = first;
                    v = (a.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                }
                if b < last {
                    c = a;
                    v = (b.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                }
                if (1 as std::ffi::c_int as std::ffi::c_long)
                    < b.offset_from(a) as std::ffi::c_long
                {
                    if ssize < 64 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1204 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10233: {
                        if ssize < 64 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1204 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if ssize < 64 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1205 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10137: {
                        if ssize < 64 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1205 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    trlink = ssize - 2 as std::ffi::c_int;
                }
                if a.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(b) as std::ffi::c_long
                {
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < a.offset_from(first) as std::ffi::c_long
                    {
                        if ssize < 64 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1210 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_10007: {
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1210 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        last = a;
                        limit = tr_ilg(
                            a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                        );
                    } else if (1 as std::ffi::c_int as std::ffi::c_long)
                        < last.offset_from(b) as std::ffi::c_long
                    {
                        first = b;
                        limit = tr_ilg(
                            last.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                        );
                    } else {
                        if 0 as std::ffi::c_int <= ssize {} else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1215 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_9863: {
                            if 0 as std::ffi::c_int <= ssize {} else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1215 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                            return;
                        }
                    }
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(b) as std::ffi::c_long
                {
                    if ssize < 64 as std::ffi::c_int {} else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1219 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_9744: {
                        if ssize < 64 as std::ffi::c_int {} else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1219 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    first = b;
                    limit = tr_ilg(
                        last.offset_from(b) as std::ffi::c_long as std::ffi::c_int,
                    );
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < a.offset_from(first) as std::ffi::c_long
                {
                    last = a;
                    limit = tr_ilg(
                        a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    );
                } else {
                    if 0 as std::ffi::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1224 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_9600: {
                        if 0 as std::ffi::c_int <= ssize {} else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1224 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                        return;
                    }
                }
            } else if limit == -(2 as std::ffi::c_int) {
                ssize -= 1;
                a = stack[ssize as usize].b;
                b = stack[ssize as usize].c;
                if stack[ssize as usize].d == 0 as std::ffi::c_int {
                    tr_copy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int,
                    );
                } else {
                    if 0 as std::ffi::c_int <= trlink {
                        stack[trlink as usize].d = -(1 as std::ffi::c_int);
                    }
                    tr_partialcopy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int,
                    );
                }
                if 0 as std::ffi::c_int <= ssize {} else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                            as *const u8 as *const std::ffi::c_char,
                        1236 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 73],
                            &[std::ffi::c_char; 73],
                        >(
                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_8904: {
                    if 0 as std::ffi::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1236 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                    return;
                }
            } else {
                if 0 as std::ffi::c_int <= *first {
                    a = first;
                    loop {
                        *ISA
                            .offset(
                                *a as isize,
                            ) = a.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                        a = a.offset(1);
                        if !(a < last && 0 as std::ffi::c_int <= *a) {
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
                        if !(*a < 0 as std::ffi::c_int) {
                            break;
                        }
                    }
                    next = if *ISA.offset(*a as isize) != *ISAd.offset(*a as isize) {
                        tr_ilg(
                            (a.offset_from(first) as std::ffi::c_long
                                + 1 as std::ffi::c_int as std::ffi::c_long)
                                as std::ffi::c_int,
                        )
                    } else {
                        -(1 as std::ffi::c_int)
                    };
                    a = a.offset(1);
                    if a < last {
                        b = first;
                        v = (a.offset_from(SA) as std::ffi::c_long
                            - 1 as std::ffi::c_int as std::ffi::c_long)
                            as std::ffi::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.offset(1);
                            b;
                        }
                    }
                    if trbudget_check(
                        budget,
                        a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    ) != 0
                    {
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(a) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1252 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_8625: {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1252 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(a) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1256 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_8505: {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1256 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            first = a;
                            limit = -(3 as std::ffi::c_int);
                        } else {
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        }
                    } else {
                        if 0 as std::ffi::c_int <= trlink {
                            stack[trlink as usize].d = -(1 as std::ffi::c_int);
                        }
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(a) as std::ffi::c_long
                        {
                            first = a;
                            limit = -(3 as std::ffi::c_int);
                        } else {
                            if 0 as std::ffi::c_int <= ssize {} else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1267 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_8341: {
                                if 0 as std::ffi::c_int <= ssize {} else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1267 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                                return;
                            }
                        }
                    }
                } else {
                    if 0 as std::ffi::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1271 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_8226: {
                        if 0 as std::ffi::c_int <= ssize {} else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1271 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                        return;
                    }
                }
            }
        } else if last.offset_from(first) as std::ffi::c_long
            <= TR_INSERTIONSORT_THRESHOLD as std::ffi::c_long
        {
            tr_insertionsort(ISAd, first, last);
            limit = -(3 as std::ffi::c_int);
        } else {
            let fresh157 = limit;
            limit = limit - 1;
            if fresh157 == 0 as std::ffi::c_int {
                tr_heapsort(
                    ISAd,
                    first,
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                );
                a = last.offset(-(1 as std::ffi::c_int as isize));
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.offset(-(1 as std::ffi::c_int as isize));
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.offset(-1);
                        b;
                    }
                    a = b;
                }
                limit = -(3 as std::ffi::c_int);
            } else {
                a = tr_pivot(ISAd, first, last);
                let ref mut fresh158 = SWAP!(* first, * a);
                *fresh158 = SWAP!(* first, * a);
                let ref mut fresh159 = SWAP!(* first, * a);
                *fresh159 = SWAP!(* first, * a);
                let ref mut fresh160 = SWAP!(* first, * a);
                *fresh160 = SWAP!(* first, * a);
                v = *ISAd.offset(*first as isize);
                tr_partition(
                    ISAd,
                    first,
                    first.offset(1 as std::ffi::c_int as isize),
                    last,
                    &mut a,
                    &mut b,
                    v,
                );
                if last.offset_from(first) as std::ffi::c_long
                    != b.offset_from(a) as std::ffi::c_long
                {
                    next = if *ISA.offset(*a as isize) != v {
                        tr_ilg(b.offset_from(a) as std::ffi::c_long as std::ffi::c_int)
                    } else {
                        -(1 as std::ffi::c_int)
                    };
                    c = first;
                    v = (a.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long) as std::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                    if b < last {
                        c = a;
                        v = (b.offset_from(SA) as std::ffi::c_long
                            - 1 as std::ffi::c_int as std::ffi::c_long)
                            as std::ffi::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.offset(1);
                            c;
                        }
                    }
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < b.offset_from(a) as std::ffi::c_long
                        && trbudget_check(
                            budget,
                            b.offset_from(a) as std::ffi::c_long as std::ffi::c_int,
                        ) != 0
                    {
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(b) as std::ffi::c_long
                        {
                            if last.offset_from(b) as std::ffi::c_long
                                <= b.offset_from(a) as std::ffi::c_long
                            {
                                if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < a.offset_from(first) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1311 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6062: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1311 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1312 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5963: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1312 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    last = a;
                                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < last.offset_from(b) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1315 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5852: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1315 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    first = b;
                                } else {
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else if a.offset_from(first) as std::ffi::c_long
                                <= b.offset_from(a) as std::ffi::c_long
                            {
                                if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < a.offset_from(first) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1322 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5700: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1322 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1323 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5604: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1323 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    last = a;
                                } else {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1326 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5498: {
                                        if ssize < 64 as std::ffi::c_int {} else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1326 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1330 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5377: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1330 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1331 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5281: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1331 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if a.offset_from(first) as std::ffi::c_long
                            <= b.offset_from(a) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1337 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5136: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1337 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1338 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5037: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1338 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                first = b;
                            } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                < a.offset_from(first) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1341 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4926: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1341 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                last = a;
                            } else {
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if last.offset_from(b) as std::ffi::c_long
                            <= b.offset_from(a) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1348 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4774: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1348 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1349 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4678: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1349 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                first = b;
                            } else {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1352 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4572: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1352 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else {
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1356 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4451: {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1356 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1357 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4355: {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1357 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            ISAd = ISAd.offset(incr as isize);
                            first = a;
                            last = b;
                            limit = next;
                        }
                    } else {
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < b.offset_from(a) as std::ffi::c_long
                            && 0 as std::ffi::c_int <= trlink
                        {
                            stack[trlink as usize].d = -(1 as std::ffi::c_int);
                        }
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(b) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < a.offset_from(first) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1365 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4178: {
                                    if ssize < 64 as std::ffi::c_int {} else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1365 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                last = a;
                            } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                first = b;
                            } else {
                                if 0 as std::ffi::c_int <= ssize {} else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1370 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4065: {
                                    if 0 as std::ffi::c_int <= ssize {} else {
                                        __assert_fail(
                                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1370 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                                    return;
                                }
                            }
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {} else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1374 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_3945: {
                                if ssize < 64 as std::ffi::c_int {} else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1374 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            first = b;
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < a.offset_from(first) as std::ffi::c_long
                        {
                            last = a;
                        } else {
                            if 0 as std::ffi::c_int <= ssize {} else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1379 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_3830: {
                                if 0 as std::ffi::c_int <= ssize {} else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1379 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                                return;
                            }
                        }
                    }
                } else if trbudget_check(
                    budget,
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                ) != 0
                {
                    limit = tr_ilg(
                        last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    );
                    ISAd = ISAd.offset(incr as isize);
                } else {
                    if 0 as std::ffi::c_int <= trlink {
                        stack[trlink as usize].d = -(1 as std::ffi::c_int);
                    }
                    if 0 as std::ffi::c_int <= ssize {} else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1388 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_3258: {
                        if 0 as std::ffi::c_int <= ssize {} else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1388 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if STACK_POP5!(ISAd, first, last, limit, trlink) != 0 {
                        return;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn trsort(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut ISAd = 0 as *mut std::ffi::c_int;
    let mut first = 0 as *mut std::ffi::c_int;
    let mut last = 0 as *mut std::ffi::c_int;
    let mut budget = _trbudget_t {
        chance: 0,
        remain: 0,
        incval: 0,
        count: 0,
    };
    let mut t: std::ffi::c_int = 0;
    let mut skip: std::ffi::c_int = 0;
    let mut unsorted: std::ffi::c_int = 0;
    trbudget_init(
        &mut budget,
        tr_ilg(n) * 2 as std::ffi::c_int / 3 as std::ffi::c_int,
        n,
    );
    ISAd = ISA.offset(depth as isize);
    while -n < *SA {
        first = SA;
        skip = 0 as std::ffi::c_int;
        unsorted = 0 as std::ffi::c_int;
        loop {
            t = *first;
            if t < 0 as std::ffi::c_int {
                first = first.offset(-(t as isize));
                skip += t;
            } else {
                if skip != 0 as std::ffi::c_int {
                    *first.offset(skip as isize) = skip;
                    skip = 0 as std::ffi::c_int;
                }
                last = SA
                    .offset(*ISA.offset(t as isize) as isize)
                    .offset(1 as std::ffi::c_int as isize);
                if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(first) as std::ffi::c_long
                {
                    budget.count = 0 as std::ffi::c_int;
                    tr_introsort(ISA, ISAd, SA, first, last, &mut budget);
                    if budget.count != 0 as std::ffi::c_int {
                        unsorted += budget.count;
                    } else {
                        skip = first.offset_from(last) as std::ffi::c_long
                            as std::ffi::c_int;
                    }
                } else if last.offset_from(first) as std::ffi::c_long
                    == 1 as std::ffi::c_int as std::ffi::c_long
                {
                    skip = -(1 as std::ffi::c_int);
                }
                first = last;
            }
            if !(first < SA.offset(n as isize)) {
                break;
            }
        }
        if skip != 0 as std::ffi::c_int {
            *first.offset(skip as isize) = skip;
        }
        if unsorted == 0 as std::ffi::c_int {
            break;
        }
        ISAd = ISAd.offset(ISAd.offset_from(ISA) as std::ffi::c_long as isize);
    }
}
unsafe extern "C" fn sort_typeBstar(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut PAb = 0 as *mut std::ffi::c_int;
    let mut ISAb = 0 as *mut std::ffi::c_int;
    let mut buf = 0 as *mut std::ffi::c_int;
    let mut i: std::ffi::c_int = 0;
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut bufsize: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < BUCKET_A_SIZE {
        *bucket_A.offset(i as isize) = 0 as std::ffi::c_int;
        i += 1;
        i;
    }
    i = 0 as std::ffi::c_int;
    while i < BUCKET_B_SIZE {
        *bucket_B.offset(i as isize) = 0 as std::ffi::c_int;
        i += 1;
        i;
    }
    i = n - 1 as std::ffi::c_int;
    m = n;
    c0 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        loop {
            let ref mut fresh161 = BUCKET_A!(c1 = c0);
            *fresh161 += 1;
            *fresh161;
            i -= 1;
            if !(0 as std::ffi::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as std::ffi::c_int;
                    c0 >= c1
                })
            {
                break;
            }
        }
        if 0 as std::ffi::c_int <= i {
            let ref mut fresh162 = BUCKET_BSTAR!(c0, c1);
            *fresh162 += 1;
            *fresh162;
            m -= 1;
            *SA.offset(m as isize) = i;
            i -= 1;
            i;
            c1 = c0;
            while 0 as std::ffi::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as std::ffi::c_int;
                    c0 <= c1
                }
            {
                let ref mut fresh163 = BUCKET_B!(c0, c1);
                *fresh163 += 1;
                *fresh163;
                i -= 1;
                i;
                c1 = c0;
            }
        }
    }
    m = n - m;
    c0 = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    j = 0 as std::ffi::c_int;
    while c0 < ALPHABET_SIZE {
        t = i + BUCKET_A!(c0);
        let ref mut fresh164 = BUCKET_A!(c0);
        *fresh164 = i + j;
        i = t + BUCKET_B!(c0, c0);
        c1 = c0 + 1 as std::ffi::c_int;
        while c1 < ALPHABET_SIZE {
            j += BUCKET_BSTAR!(c0, c1);
            let ref mut fresh165 = BUCKET_BSTAR!(c0, c1);
            *fresh165 = j;
            i += BUCKET_B!(c0, c1);
            c1 += 1;
            c1;
        }
        c0 += 1;
        c0;
    }
    if (0 as std::ffi::c_int) < m {
        PAb = SA.offset(n as isize).offset(-(m as isize));
        ISAb = SA.offset(m as isize);
        i = m - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            t = *PAb.offset(i as isize);
            c0 = *T.offset(t as isize) as std::ffi::c_int;
            c1 = *T.offset((t + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
            let ref mut fresh166 = BUCKET_BSTAR!(c0, c1);
            *fresh166 -= 1;
            *SA.offset(*fresh166 as isize) = i;
            i -= 1;
            i;
        }
        t = *PAb.offset((m - 1 as std::ffi::c_int) as isize);
        c0 = *T.offset(t as isize) as std::ffi::c_int;
        c1 = *T.offset((t + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        let ref mut fresh167 = BUCKET_BSTAR!(c0, c1);
        *fresh167 -= 1;
        *SA.offset(*fresh167 as isize) = m - 1 as std::ffi::c_int;
        buf = SA.offset(m as isize);
        bufsize = n - 2 as std::ffi::c_int * m;
        c0 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        j = m;
        while (0 as std::ffi::c_int) < j {
            c1 = ALPHABET_SIZE - 1 as std::ffi::c_int;
            while c0 < c1 {
                i = BUCKET_BSTAR!(c0, c1);
                if (1 as std::ffi::c_int) < j - i {
                    sssort(
                        T,
                        PAb,
                        SA.offset(i as isize),
                        SA.offset(j as isize),
                        buf,
                        bufsize,
                        2 as std::ffi::c_int,
                        n,
                        (*SA.offset(i as isize) == m - 1 as std::ffi::c_int)
                            as std::ffi::c_int,
                    );
                }
                j = i;
                c1 -= 1;
                c1;
            }
            c0 -= 1;
            c0;
        }
        i = m - 1 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            if 0 as std::ffi::c_int <= *SA.offset(i as isize) {
                j = i;
                loop {
                    *ISAb.offset(*SA.offset(i as isize) as isize) = i;
                    i -= 1;
                    if !(0 as std::ffi::c_int <= i
                        && 0 as std::ffi::c_int <= *SA.offset(i as isize))
                    {
                        break;
                    }
                }
                *SA.offset((i + 1 as std::ffi::c_int) as isize) = i - j;
                if i <= 0 as std::ffi::c_int {
                    break;
                }
            }
            j = i;
            loop {
                let ref mut fresh168 = *SA.offset(i as isize);
                *fresh168 = !*SA.offset(i as isize);
                *ISAb.offset(*fresh168 as isize) = j;
                i -= 1;
                if !(*SA.offset(i as isize) < 0 as std::ffi::c_int) {
                    break;
                }
            }
            *ISAb.offset(*SA.offset(i as isize) as isize) = j;
            i -= 1;
            i;
        }
        trsort(ISAb, SA, m, 1 as std::ffi::c_int);
        i = n - 1 as std::ffi::c_int;
        j = m;
        c0 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            i -= 1;
            i;
            c1 = c0;
            while 0 as std::ffi::c_int <= i
                && {
                    c0 = *T.offset(i as isize) as std::ffi::c_int;
                    c0 >= c1
                }
            {
                i -= 1;
                i;
                c1 = c0;
            }
            if 0 as std::ffi::c_int <= i {
                t = i;
                i -= 1;
                i;
                c1 = c0;
                while 0 as std::ffi::c_int <= i
                    && {
                        c0 = *T.offset(i as isize) as std::ffi::c_int;
                        c0 <= c1
                    }
                {
                    i -= 1;
                    i;
                    c1 = c0;
                }
                j -= 1;
                *SA
                    .offset(
                        *ISAb.offset(j as isize) as isize,
                    ) = if t == 0 as std::ffi::c_int || (1 as std::ffi::c_int) < t - i {
                    t
                } else {
                    !t
                };
            }
        }
        let ref mut fresh169 = BUCKET_B!(ALPHABET_SIZE - 1, ALPHABET_SIZE - 1);
        *fresh169 = n;
        c0 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        k = m - 1 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c0 {
            i = BUCKET_A!(c0 + 1) - 1 as std::ffi::c_int;
            c1 = ALPHABET_SIZE - 1 as std::ffi::c_int;
            while c0 < c1 {
                t = i - BUCKET_B!(c0, c1);
                let ref mut fresh170 = BUCKET_B!(c0, c1);
                *fresh170 = i;
                i = t;
                j = BUCKET_BSTAR!(c0, c1);
                while j <= k {
                    *SA.offset(i as isize) = *SA.offset(k as isize);
                    i -= 1;
                    i;
                    k -= 1;
                    k;
                }
                c1 -= 1;
                c1;
            }
            let ref mut fresh171 = BUCKET_BSTAR!(c0, c0 + 1);
            *fresh171 = i - BUCKET_B!(c0, c0) + 1 as std::ffi::c_int;
            let ref mut fresh172 = BUCKET_B!(c0, c0);
            *fresh172 = i;
            c0 -= 1;
            c0;
        }
    }
    return m;
}
unsafe extern "C" fn construct_SA(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
) {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(BUCKET_BSTAR!(c1, c1 + 1) as isize);
            j = SA
                .offset(BUCKET_A!(c1 + 1) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                as std::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1631 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2348: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                    as std::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1631 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    *j = !s;
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize)
                            as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            let ref mut fresh173 = BUCKET_B!(c2, c1);
                            *fresh173 = k.offset_from(SA) as std::ffi::c_long
                                as std::ffi::c_int;
                        }
                        k = SA.offset(BUCKET_B!(c2 = c0, c1) as isize);
                    }
                    let fresh174 = k;
                    k = k.offset(-1);
                    *fresh174 = s;
                } else {
                    if s == 0 as std::ffi::c_int
                        && *T.offset(s as isize) as std::ffi::c_int == c1
                        || s < 0 as std::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"((s == 0) && (T[s] == c1)) || (s < 0)\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1643 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2011: {
                        if s == 0 as std::ffi::c_int
                            && *T.offset(s as isize) as std::ffi::c_int == c1
                            || s < 0 as std::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"((s == 0) && (T[s] == c1)) || (s < 0)\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1643 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    *j = !s;
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    k = SA.offset(BUCKET_A!(c2 = T[n - 1]) as isize);
    let fresh175 = k;
    k = k.offset(1);
    *fresh175 = if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int)
        < c2
    {
        !(n - 1 as std::ffi::c_int)
    } else {
        n - 1 as std::ffi::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            if s == 0 as std::ffi::c_int
                || (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int)
                    < c0
            {
                s = !s;
            }
            if c0 != c2 {
                let ref mut fresh176 = BUCKET_A!(c2);
                *fresh176 = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                k = SA.offset(BUCKET_A!(c2 = c0) as isize);
            }
            let fresh177 = k;
            k = k.offset(1);
            *fresh177 = s;
        } else {
            *i = !s;
        }
        i = i.offset(1);
        i;
    }
}
unsafe extern "C" fn construct_BWT(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut orig = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(BUCKET_BSTAR!(c1, c1 + 1) as isize);
            j = SA
                .offset(BUCKET_A!(c1 + 1) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                as std::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1695 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21583: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                    as std::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1695 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    *j = !c0;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize)
                            as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            let ref mut fresh178 = BUCKET_B!(c2, c1);
                            *fresh178 = k.offset_from(SA) as std::ffi::c_long
                                as std::ffi::c_int;
                        }
                        k = SA.offset(BUCKET_B!(c2 = c0, c1) as isize);
                    }
                    let fresh179 = k;
                    k = k.offset(-1);
                    *fresh179 = s;
                } else if s != 0 as std::ffi::c_int {
                    *j = !s;
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    k = SA.offset(BUCKET_A!(c2 = T[n - 1]) as isize);
    let fresh180 = k;
    k = k.offset(1);
    *fresh180 = if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int)
        < c2
    {
        !(*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int)
    } else {
        n - 1 as std::ffi::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            *i = c0;
            if (0 as std::ffi::c_int) < s
                && (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int)
                    < c0
            {
                s = !(*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int);
            }
            if c0 != c2 {
                let ref mut fresh181 = BUCKET_A!(c2);
                *fresh181 = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                k = SA.offset(BUCKET_A!(c2 = c0) as isize);
            }
            let fresh182 = k;
            k = k.offset(1);
            *fresh182 = s;
        } else if s != 0 as std::ffi::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
        i;
    }
    return orig.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
}
unsafe extern "C" fn construct_BWT_indexes(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
    mut num_indexes: *mut std::ffi::c_uchar,
    mut indexes: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut orig = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    let mut mod_0 = n / 8 as std::ffi::c_int;
    mod_0 |= mod_0 >> 1 as std::ffi::c_int;
    mod_0 |= mod_0 >> 2 as std::ffi::c_int;
    mod_0 |= mod_0 >> 4 as std::ffi::c_int;
    mod_0 |= mod_0 >> 8 as std::ffi::c_int;
    mod_0 |= mod_0 >> 16 as std::ffi::c_int;
    mod_0 >>= 1 as std::ffi::c_int;
    *num_indexes = ((n - 1 as std::ffi::c_int) / (mod_0 + 1 as std::ffi::c_int))
        as std::ffi::c_uchar;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(BUCKET_BSTAR!(c1, c1 + 1) as isize);
            j = SA
                .offset(BUCKET_A!(c1 + 1) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                as std::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1776 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_20622: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize)
                                    as std::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/home/peter/Dev/zstd-c2rust/lib/dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1776 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if s & mod_0 == 0 as std::ffi::c_int {
                        *indexes
                            .offset(
                                (s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int)
                                    as isize,
                            ) = j.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    *j = !c0;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize)
                            as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            let ref mut fresh183 = BUCKET_B!(c2, c1);
                            *fresh183 = k.offset_from(SA) as std::ffi::c_long
                                as std::ffi::c_int;
                        }
                        k = SA.offset(BUCKET_B!(c2 = c0, c1) as isize);
                    }
                    let fresh184 = k;
                    k = k.offset(-1);
                    *fresh184 = s;
                } else if s != 0 as std::ffi::c_int {
                    *j = !s;
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    k = SA.offset(BUCKET_A!(c2 = T[n - 1]) as isize);
    if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int) < c2 {
        if n - 1 as std::ffi::c_int & mod_0 == 0 as std::ffi::c_int {
            *indexes
                .offset(
                    ((n - 1 as std::ffi::c_int) / (mod_0 + 1 as std::ffi::c_int)
                        - 1 as std::ffi::c_int) as isize,
                ) = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        let fresh185 = k;
        k = k.offset(1);
        *fresh185 = !(*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int);
    } else {
        let fresh186 = k;
        k = k.offset(1);
        *fresh186 = n - 1 as std::ffi::c_int;
    }
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            if s & mod_0 == 0 as std::ffi::c_int {
                *indexes
                    .offset(
                        (s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int)
                            as isize,
                    ) = i.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            *i = c0;
            if c0 != c2 {
                let ref mut fresh187 = BUCKET_A!(c2);
                *fresh187 = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                k = SA.offset(BUCKET_A!(c2 = c0) as isize);
            }
            if (0 as std::ffi::c_int) < s
                && (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int)
                    < c0
            {
                if s & mod_0 == 0 as std::ffi::c_int {
                    *indexes
                        .offset(
                            (s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int)
                                as isize,
                        ) = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                }
                let fresh188 = k;
                k = k.offset(1);
                *fresh188 = !(*T.offset((s - 1 as std::ffi::c_int) as isize)
                    as std::ffi::c_int);
            } else {
                let fresh189 = k;
                k = k.offset(1);
                *fresh189 = s;
            }
        } else if s != 0 as std::ffi::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
        i;
    }
    return orig.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn divsufsort(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut bucket_A = 0 as *mut std::ffi::c_int;
    let mut bucket_B = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut err: std::ffi::c_int = 0;
    if T.is_null() || SA.is_null() || n < 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int)
    } else if n == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int
    } else if n == 1 as std::ffi::c_int {
        *SA.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int;
        return 0 as std::ffi::c_int;
    } else if n == 2 as std::ffi::c_int {
        m = ((*T.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int)
            < *T.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
            as std::ffi::c_int;
        *SA.offset((m ^ 1 as std::ffi::c_int) as isize) = 0 as std::ffi::c_int;
        *SA.offset(m as isize) = 1 as std::ffi::c_int;
        return 0 as std::ffi::c_int;
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>()),
    ) as *mut std::ffi::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>()),
    ) as *mut std::ffi::c_int;
    if !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, SA, bucket_A, bucket_B, n, openMP);
        construct_SA(T, SA, bucket_A, bucket_B, n, m);
    } else {
        err = -(2 as std::ffi::c_int);
    }
    free(bucket_B as *mut std::ffi::c_void);
    free(bucket_A as *mut std::ffi::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn divbwt(
    mut T: *const std::ffi::c_uchar,
    mut U: *mut std::ffi::c_uchar,
    mut A: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut num_indexes: *mut std::ffi::c_uchar,
    mut indexes: *mut std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut B = 0 as *mut std::ffi::c_int;
    let mut bucket_A = 0 as *mut std::ffi::c_int;
    let mut bucket_B = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut pidx: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    if T.is_null() || U.is_null() || n < 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int)
    } else if n <= 1 as std::ffi::c_int {
        if n == 1 as std::ffi::c_int {
            *U
                .offset(
                    0 as std::ffi::c_int as isize,
                ) = *T.offset(0 as std::ffi::c_int as isize);
        }
        return n;
    }
    B = A;
    if B.is_null() {
        B = malloc(
            ((n + 1 as std::ffi::c_int) as usize)
                .wrapping_mul(
                    ::core::mem::size_of::<std::ffi::c_int>(),
                ),
        ) as *mut std::ffi::c_int;
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>()),
    ) as *mut std::ffi::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>()),
    ) as *mut std::ffi::c_int;
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
                0 as std::ffi::c_int as isize,
            ) = *T.offset((n - 1 as std::ffi::c_int) as isize);
        i = 0 as std::ffi::c_int;
        while i < pidx {
            *U
                .offset(
                    (i + 1 as std::ffi::c_int) as isize,
                ) = *B.offset(i as isize) as std::ffi::c_uchar;
            i += 1;
            i;
        }
        i += 1 as std::ffi::c_int;
        while i < n {
            *U.offset(i as isize) = *B.offset(i as isize) as std::ffi::c_uchar;
            i += 1;
            i;
        }
        pidx += 1 as std::ffi::c_int;
    } else {
        pidx = -(2 as std::ffi::c_int);
    }
    free(bucket_B as *mut std::ffi::c_void);
    free(bucket_A as *mut std::ffi::c_void);
    if A.is_null() {
        free(B as *mut std::ffi::c_void);
    }
    return pidx;
}
