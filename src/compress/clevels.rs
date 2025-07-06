use crate::zstd_h::*;

/*-=====  Pre-defined compression levels  =====-*/
pub const ZSTD_MAX_CLEVEL: i32 = 22;

pub fn ZSTD_maxCLevel() -> i32 {
    return ZSTD_MAX_CLEVEL;
}
pub fn ZSTD_minCLevel() -> i32 {
    return -ZSTD_TARGETLENGTH_MAX;
}
pub fn ZSTD_defaultCLevel() -> i32 {
    return ZSTD_CLEVEL_DEFAULT;
}

macro_rules! params {
    ($w:expr, $c:expr, $h:expr, $s:expr, $l:expr, $tl:expr, $strat:expr) => {
        ZSTD_compressionParameters {
            windowLog: $w,
            chainLog: $c,
            hashLog: $h,
            searchLog: $s,
            minMatch: $l,
            targetLength: $tl,
            strategy: $strat,
        }
    }
}

#[rustfmt::skip]
pub const ZSTD_defaultCParameters: [[ZSTD_compressionParameters; ZSTD_MAX_CLEVEL as usize + 1]; 4] = [
[   /* "default" - for any srcSize > 256 KB */
    /* W,  C,  H,  S,  L, TL, strat */
    params!{ 19, 12, 13,  1,  6,  1, ZSTD_fast    },  /* base for negative levels */
    params!{ 19, 13, 14,  1,  7,  0, ZSTD_fast    },  /* level  1 */
    params!{ 20, 15, 16,  1,  6,  0, ZSTD_fast    },  /* level  2 */
    params!{ 21, 16, 17,  1,  5,  0, ZSTD_dfast   },  /* level  3 */
    params!{ 21, 18, 18,  1,  5,  0, ZSTD_dfast   },  /* level  4 */
    params!{ 21, 18, 19,  3,  5,  2, ZSTD_greedy  },  /* level  5 */
    params!{ 21, 18, 19,  3,  5,  4, ZSTD_lazy    },  /* level  6 */
    params!{ 21, 19, 20,  4,  5,  8, ZSTD_lazy    },  /* level  7 */
    params!{ 21, 19, 20,  4,  5, 16, ZSTD_lazy2   },  /* level  8 */
    params!{ 22, 20, 21,  4,  5, 16, ZSTD_lazy2   },  /* level  9 */
    params!{ 22, 21, 22,  5,  5, 16, ZSTD_lazy2   },  /* level 10 */
    params!{ 22, 21, 22,  6,  5, 16, ZSTD_lazy2   },  /* level 11 */
    params!{ 22, 22, 23,  6,  5, 32, ZSTD_lazy2   },  /* level 12 */
    params!{ 22, 22, 22,  4,  5, 32, ZSTD_btlazy2 },  /* level 13 */
    params!{ 22, 22, 23,  5,  5, 32, ZSTD_btlazy2 },  /* level 14 */
    params!{ 22, 23, 23,  6,  5, 32, ZSTD_btlazy2 },  /* level 15 */
    params!{ 22, 22, 22,  5,  5, 48, ZSTD_btopt   },  /* level 16 */
    params!{ 23, 23, 22,  5,  4, 64, ZSTD_btopt   },  /* level 17 */
    params!{ 23, 23, 22,  6,  3, 64, ZSTD_btultra },  /* level 18 */
    params!{ 23, 24, 22,  7,  3,256, ZSTD_btultra2},  /* level 19 */
    params!{ 25, 25, 23,  7,  3,256, ZSTD_btultra2},  /* level 20 */
    params!{ 26, 26, 24,  7,  3,512, ZSTD_btultra2},  /* level 21 */
    params!{ 27, 27, 25,  9,  3,999, ZSTD_btultra2},  /* level 22 */
],
[   /* for srcSize <= 256 KB */
    /* W,  C,  H,  S,  L,  T, strat */
    params!{ 18, 12, 13,  1,  5,  1, ZSTD_fast    },  /* base for negative levels */
    params!{ 18, 13, 14,  1,  6,  0, ZSTD_fast    },  /* level  1 */
    params!{ 18, 14, 14,  1,  5,  0, ZSTD_dfast   },  /* level  2 */
    params!{ 18, 16, 16,  1,  4,  0, ZSTD_dfast   },  /* level  3 */
    params!{ 18, 16, 17,  3,  5,  2, ZSTD_greedy  },  /* level  4.*/
    params!{ 18, 17, 18,  5,  5,  2, ZSTD_greedy  },  /* level  5.*/
    params!{ 18, 18, 19,  3,  5,  4, ZSTD_lazy    },  /* level  6.*/
    params!{ 18, 18, 19,  4,  4,  4, ZSTD_lazy    },  /* level  7 */
    params!{ 18, 18, 19,  4,  4,  8, ZSTD_lazy2   },  /* level  8 */
    params!{ 18, 18, 19,  5,  4,  8, ZSTD_lazy2   },  /* level  9 */
    params!{ 18, 18, 19,  6,  4,  8, ZSTD_lazy2   },  /* level 10 */
    params!{ 18, 18, 19,  5,  4, 12, ZSTD_btlazy2 },  /* level 11.*/
    params!{ 18, 19, 19,  7,  4, 12, ZSTD_btlazy2 },  /* level 12.*/
    params!{ 18, 18, 19,  4,  4, 16, ZSTD_btopt   },  /* level 13 */
    params!{ 18, 18, 19,  4,  3, 32, ZSTD_btopt   },  /* level 14.*/
    params!{ 18, 18, 19,  6,  3,128, ZSTD_btopt   },  /* level 15.*/
    params!{ 18, 19, 19,  6,  3,128, ZSTD_btultra },  /* level 16.*/
    params!{ 18, 19, 19,  8,  3,256, ZSTD_btultra },  /* level 17.*/
    params!{ 18, 19, 19,  6,  3,128, ZSTD_btultra2},  /* level 18.*/
    params!{ 18, 19, 19,  8,  3,256, ZSTD_btultra2},  /* level 19.*/
    params!{ 18, 19, 19, 10,  3,512, ZSTD_btultra2},  /* level 20.*/
    params!{ 18, 19, 19, 12,  3,512, ZSTD_btultra2},  /* level 21.*/
    params!{ 18, 19, 19, 13,  3,999, ZSTD_btultra2},  /* level 22.*/
],
[   /* for srcSize <= 128 KB */
    /* W,  C,  H,  S,  L,  T, strat */
    params!{ 17, 12, 12,  1,  5,  1, ZSTD_fast    },  /* base for negative levels */
    params!{ 17, 12, 13,  1,  6,  0, ZSTD_fast    },  /* level  1 */
    params!{ 17, 13, 15,  1,  5,  0, ZSTD_fast    },  /* level  2 */
    params!{ 17, 15, 16,  2,  5,  0, ZSTD_dfast   },  /* level  3 */
    params!{ 17, 17, 17,  2,  4,  0, ZSTD_dfast   },  /* level  4 */
    params!{ 17, 16, 17,  3,  4,  2, ZSTD_greedy  },  /* level  5 */
    params!{ 17, 16, 17,  3,  4,  4, ZSTD_lazy    },  /* level  6 */
    params!{ 17, 16, 17,  3,  4,  8, ZSTD_lazy2   },  /* level  7 */
    params!{ 17, 16, 17,  4,  4,  8, ZSTD_lazy2   },  /* level  8 */
    params!{ 17, 16, 17,  5,  4,  8, ZSTD_lazy2   },  /* level  9 */
    params!{ 17, 16, 17,  6,  4,  8, ZSTD_lazy2   },  /* level 10 */
    params!{ 17, 17, 17,  5,  4,  8, ZSTD_btlazy2 },  /* level 11 */
    params!{ 17, 18, 17,  7,  4, 12, ZSTD_btlazy2 },  /* level 12 */
    params!{ 17, 18, 17,  3,  4, 12, ZSTD_btopt   },  /* level 13.*/
    params!{ 17, 18, 17,  4,  3, 32, ZSTD_btopt   },  /* level 14.*/
    params!{ 17, 18, 17,  6,  3,256, ZSTD_btopt   },  /* level 15.*/
    params!{ 17, 18, 17,  6,  3,128, ZSTD_btultra },  /* level 16.*/
    params!{ 17, 18, 17,  8,  3,256, ZSTD_btultra },  /* level 17.*/
    params!{ 17, 18, 17, 10,  3,512, ZSTD_btultra },  /* level 18.*/
    params!{ 17, 18, 17,  5,  3,256, ZSTD_btultra2},  /* level 19.*/
    params!{ 17, 18, 17,  7,  3,512, ZSTD_btultra2},  /* level 20.*/
    params!{ 17, 18, 17,  9,  3,512, ZSTD_btultra2},  /* level 21.*/
    params!{ 17, 18, 17, 11,  3,999, ZSTD_btultra2},  /* level 22.*/
],
[   /* for srcSize <= 16 KB */
    /* W,  C,  H,  S,  L,  T, strat */
    params!{ 14, 12, 13,  1,  5,  1, ZSTD_fast    },  /* base for negative levels */
    params!{ 14, 14, 15,  1,  5,  0, ZSTD_fast    },  /* level  1 */
    params!{ 14, 14, 15,  1,  4,  0, ZSTD_fast    },  /* level  2 */
    params!{ 14, 14, 15,  2,  4,  0, ZSTD_dfast   },  /* level  3 */
    params!{ 14, 14, 14,  4,  4,  2, ZSTD_greedy  },  /* level  4 */
    params!{ 14, 14, 14,  3,  4,  4, ZSTD_lazy    },  /* level  5.*/
    params!{ 14, 14, 14,  4,  4,  8, ZSTD_lazy2   },  /* level  6 */
    params!{ 14, 14, 14,  6,  4,  8, ZSTD_lazy2   },  /* level  7 */
    params!{ 14, 14, 14,  8,  4,  8, ZSTD_lazy2   },  /* level  8.*/
    params!{ 14, 15, 14,  5,  4,  8, ZSTD_btlazy2 },  /* level  9.*/
    params!{ 14, 15, 14,  9,  4,  8, ZSTD_btlazy2 },  /* level 10.*/
    params!{ 14, 15, 14,  3,  4, 12, ZSTD_btopt   },  /* level 11.*/
    params!{ 14, 15, 14,  4,  3, 24, ZSTD_btopt   },  /* level 12.*/
    params!{ 14, 15, 14,  5,  3, 32, ZSTD_btultra },  /* level 13.*/
    params!{ 14, 15, 15,  6,  3, 64, ZSTD_btultra },  /* level 14.*/
    params!{ 14, 15, 15,  7,  3,256, ZSTD_btultra },  /* level 15.*/
    params!{ 14, 15, 15,  5,  3, 48, ZSTD_btultra2},  /* level 16.*/
    params!{ 14, 15, 15,  6,  3,128, ZSTD_btultra2},  /* level 17.*/
    params!{ 14, 15, 15,  7,  3,256, ZSTD_btultra2},  /* level 18.*/
    params!{ 14, 15, 15,  8,  3,256, ZSTD_btultra2},  /* level 19.*/
    params!{ 14, 15, 15,  8,  3,512, ZSTD_btultra2},  /* level 20.*/
    params!{ 14, 15, 15,  9,  3,512, ZSTD_btultra2},  /* level 21.*/
    params!{ 14, 15, 15, 10,  3,999, ZSTD_btultra2},  /* level 22.*/
],
];
