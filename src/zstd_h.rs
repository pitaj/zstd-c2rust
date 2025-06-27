pub const fn ZSTD_FRAMEHEADERSIZE_MIN(format: ZSTD_format_e) {
    if format == ZSTD_f_zstd1 {
        6
    } else {
        2
    }
}

pub const fn ZSTD_FRAMEHEADERSIZE_PREFIX(format: ZSTD_format_e) -> usize {
    if format == ZSTD_f_zstd1 {
        5
    } else {
        1
    }
}
