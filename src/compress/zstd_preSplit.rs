use ::libc;
extern "C" {
    fn HIST_add(
        count: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: usize,
    );
}
pub type unalign16 = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fingerprint {
    pub events: [std::ffi::c_uint; 1024],
    pub nbEvents: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}
pub type RecordEvents_f = Option::<
    unsafe extern "C" fn(*mut Fingerprint, *const std::ffi::c_void, usize) -> (),
>;
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const std::ffi::c_void) -> u16 {
    return *(ptr as *const unalign16);
}
pub const THRESHOLD_PENALTY_RATE: std::ffi::c_int = 16 as std::ffi::c_int;
pub const THRESHOLD_BASE: std::ffi::c_int = THRESHOLD_PENALTY_RATE
    - 2 as std::ffi::c_int;
pub const THRESHOLD_PENALTY: std::ffi::c_int = 3 as std::ffi::c_int;
pub const HASHLENGTH: std::ffi::c_int = 2 as std::ffi::c_int;
pub const HASHLOG_MAX: std::ffi::c_int = 10 as std::ffi::c_int;
pub const HASHTABLESIZE: std::ffi::c_int = (1 as std::ffi::c_int) << HASHLOG_MAX;
pub const KNUTH: std::ffi::c_uint = 0x9e3779b9 as std::ffi::c_uint;
#[inline(always)]
unsafe extern "C" fn hash2(
    mut p: *const std::ffi::c_void,
    mut hashLog: std::ffi::c_uint,
) -> std::ffi::c_uint {
    if hashLog == 8 as std::ffi::c_int as std::ffi::c_uint {
        return *(p as *const u8).offset(0) as u32;
    }
    return (MEM_read16(p) as u32).wrapping_mul(KNUTH)
        >> (32 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(hashLog);
}
unsafe extern "C" fn initStats(mut fpstats: *mut FPStats) {
    libc::memset(
        fpstats as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<FPStats>() as usize,
    );
}
#[inline(always)]
unsafe extern "C" fn addEvents_generic(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut samplingRate: usize,
    mut hashLog: std::ffi::c_uint,
) {
    let mut p = src as *const std::ffi::c_char;
    let mut limit = srcSize
        .wrapping_sub(HASHLENGTH as usize)
        .wrapping_add(1);
    let mut n: usize = 0;
    n = 0 as std::ffi::c_int as usize;
    while n < limit {
        (*fp)
            .events[hash2(p.offset(n as isize) as *const std::ffi::c_void, hashLog)
            as usize] = ((*fp)
            .events[hash2(p.offset(n as isize) as *const std::ffi::c_void, hashLog)
            as usize])
            .wrapping_add(1);
        (*fp)
            .events[hash2(p.offset(n as isize) as *const std::ffi::c_void, hashLog)
            as usize];
        n = n.wrapping_add(samplingRate);
    }
    (*fp).nbEvents = ((*fp).nbEvents).wrapping_add(limit / samplingRate);
}
#[inline(always)]
unsafe extern "C" fn recordFingerprint_generic(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
    mut samplingRate: usize,
    mut hashLog: std::ffi::c_uint,
) {
    libc::memset(
        fp as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<std::ffi::c_uint>())
            .wrapping_mul(1_usize << hashLog) as usize,
    );
    (*fp).nbEvents = 0 as std::ffi::c_int as usize;
    addEvents_generic(fp, src, srcSize, samplingRate, hashLog);
}
unsafe extern "C" fn ZSTD_recordFingerprint_1(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) {
    ZSTD_GEN_RECORD_FINGERPRINT!(
        1, 10
    )(
        ZSTD_GEN_RECORD_FINGERPRINT!(1, 10),
        ZSTD_GEN_RECORD_FINGERPRINT!(1, 10),
        ZSTD_GEN_RECORD_FINGERPRINT!(1, 10),
        1 as std::ffi::c_int as usize,
        10 as std::ffi::c_int as std::ffi::c_uint,
    );
}
unsafe extern "C" fn ZSTD_recordFingerprint_5(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) {
    ZSTD_GEN_RECORD_FINGERPRINT!(
        5, 10
    )(
        ZSTD_GEN_RECORD_FINGERPRINT!(5, 10),
        ZSTD_GEN_RECORD_FINGERPRINT!(5, 10),
        ZSTD_GEN_RECORD_FINGERPRINT!(5, 10),
        5 as std::ffi::c_int as usize,
        10 as std::ffi::c_int as std::ffi::c_uint,
    );
}
unsafe extern "C" fn ZSTD_recordFingerprint_11(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) {
    ZSTD_GEN_RECORD_FINGERPRINT!(
        11, 9
    )(
        ZSTD_GEN_RECORD_FINGERPRINT!(11, 9),
        ZSTD_GEN_RECORD_FINGERPRINT!(11, 9),
        ZSTD_GEN_RECORD_FINGERPRINT!(11, 9),
        11 as std::ffi::c_int as usize,
        9 as std::ffi::c_int as std::ffi::c_uint,
    );
}
unsafe extern "C" fn ZSTD_recordFingerprint_43(
    mut fp: *mut Fingerprint,
    mut src: *const std::ffi::c_void,
    mut srcSize: usize,
) {
    ZSTD_GEN_RECORD_FINGERPRINT!(
        43, 8
    )(
        ZSTD_GEN_RECORD_FINGERPRINT!(43, 8),
        ZSTD_GEN_RECORD_FINGERPRINT!(43, 8),
        ZSTD_GEN_RECORD_FINGERPRINT!(43, 8),
        43 as std::ffi::c_int as usize,
        8 as std::ffi::c_int as std::ffi::c_uint,
    );
}
unsafe extern "C" fn abs64(mut s64: i64) -> u64 {
    return (if s64 < 0 as std::ffi::c_int as i64 { -s64 } else { s64 }) as u64;
}
unsafe extern "C" fn fpDistance(
    mut fp1: *const Fingerprint,
    mut fp2: *const Fingerprint,
    mut hashLog: std::ffi::c_uint,
) -> u64 {
    let mut distance: u64 = 0;
    let mut n: usize = 0;
    n = 0 as std::ffi::c_int as usize;
    while n < 1_usize << hashLog {
        distance = distance
            .wrapping_add(
                abs64(
                    (*fp1).events[n as usize] as i64 * (*fp2).nbEvents as i64
                        - (*fp2).events[n as usize] as i64 * (*fp1).nbEvents as i64,
                ),
            );
        n = n.wrapping_add(1);
        n;
    }
    return distance;
}
unsafe extern "C" fn compareFingerprints(
    mut ref_0: *const Fingerprint,
    mut newfp: *const Fingerprint,
    mut penalty: std::ffi::c_int,
    mut hashLog: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut p50 = (*ref_0).nbEvents * (*newfp).nbEvents;
    let mut deviation = fpDistance(ref_0, newfp, hashLog);
    let mut threshold = p50 * (THRESHOLD_BASE + penalty) as u64
        / THRESHOLD_PENALTY_RATE as u64;
    return (deviation >= threshold) as std::ffi::c_int;
}
unsafe extern "C" fn mergeEvents(
    mut acc: *mut Fingerprint,
    mut newfp: *const Fingerprint,
) {
    let mut n: usize = 0;
    n = 0 as std::ffi::c_int as usize;
    while n < HASHTABLESIZE as usize {
        (*acc)
            .events[n
            as usize] = ((*acc).events[n as usize])
            .wrapping_add((*newfp).events[n as usize]);
        n = n.wrapping_add(1);
        n;
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_add((*newfp).nbEvents);
}
unsafe extern "C" fn flushEvents(mut fpstats: *mut FPStats) {
    let mut n: usize = 0;
    n = 0 as std::ffi::c_int as usize;
    while n < HASHTABLESIZE as usize {
        (*fpstats)
            .pastEvents
            .events[n as usize] = (*fpstats).newEvents.events[n as usize];
        n = n.wrapping_add(1);
        n;
    }
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    libc::memset(
        &mut (*fpstats).newEvents as *mut Fingerprint as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<Fingerprint>() as usize,
    );
}
unsafe extern "C" fn removeEvents(
    mut acc: *mut Fingerprint,
    mut slice: *const Fingerprint,
) {
    let mut n: usize = 0;
    n = 0 as std::ffi::c_int as usize;
    while n < HASHTABLESIZE as usize {
        (*acc)
            .events[n
            as usize] = ((*acc).events[n as usize])
            .wrapping_sub((*slice).events[n as usize]);
        n = n.wrapping_add(1);
        n;
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_sub((*slice).nbEvents);
}
pub const CHUNKSIZE: std::ffi::c_int = (8 as std::ffi::c_int) << 10;
unsafe extern "C" fn ZSTD_splitBlock_byChunks(
    mut blockStart: *const std::ffi::c_void,
    mut blockSize: usize,
    mut level: std::ffi::c_int,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    static mut records_fs: [RecordEvents_f; 4] = unsafe {
        [FP_RECORD!(43), FP_RECORD!(11), FP_RECORD!(5), FP_RECORD!(1)]
    };
    static mut hashParams: [std::ffi::c_uint; 4] = [
        8 as std::ffi::c_int as std::ffi::c_uint,
        9 as std::ffi::c_int as std::ffi::c_uint,
        10 as std::ffi::c_int as std::ffi::c_uint,
        10 as std::ffi::c_int as std::ffi::c_uint,
    ];
    let record_f: RecordEvents_f = records_fs[level as usize];
    let fpstats = workspace as *mut FPStats;
    let mut p = blockStart as *const std::ffi::c_char;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos: usize = 0;
    initStats(fpstats);
    record_f
        .expect(
            "non-null function pointer",
        )(&mut (*fpstats).pastEvents, p as *const std::ffi::c_void, CHUNKSIZE as usize);
    pos = CHUNKSIZE as usize;
    while pos <= blockSize.wrapping_sub(CHUNKSIZE as usize) {
        record_f
            .expect(
                "non-null function pointer",
            )(
            &mut (*fpstats).newEvents,
            p.offset(pos as isize) as *const std::ffi::c_void,
            CHUNKSIZE as usize,
        );
        if compareFingerprints(
            &mut (*fpstats).pastEvents,
            &mut (*fpstats).newEvents,
            penalty,
            hashParams[level as usize],
        ) != 0
        {
            return pos
        } else {
            mergeEvents(&mut (*fpstats).pastEvents, &mut (*fpstats).newEvents);
            if penalty > 0 as std::ffi::c_int {
                penalty -= 1;
                penalty;
            }
        }
        pos = pos.wrapping_add(CHUNKSIZE as usize);
    }
    return blockSize;
}
unsafe extern "C" fn ZSTD_splitBlock_fromBorders(
    mut blockStart: *const std::ffi::c_void,
    mut blockSize: usize,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    let fpstats = workspace as *mut FPStats;
    let mut middleEvents = (workspace as *mut std::ffi::c_char)
        .offset(
            (512 as std::ffi::c_int as std::ffi::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<std::ffi::c_uint>(),
                ) as isize,
        ) as *mut std::ffi::c_void as *mut Fingerprint;
    initStats(fpstats);
    HIST_add(
        ((*fpstats).pastEvents.events).as_mut_ptr(),
        blockStart,
        SEGMENT_SIZE as usize,
    );
    HIST_add(
        ((*fpstats).newEvents.events).as_mut_ptr(),
        (blockStart as *const std::ffi::c_char)
            .offset(blockSize as isize)
            .offset(-(SEGMENT_SIZE as isize)) as *const std::ffi::c_void,
        SEGMENT_SIZE as usize,
    );
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE as usize;
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    if compareFingerprints(
        &mut (*fpstats).pastEvents,
        &mut (*fpstats).newEvents,
        0 as std::ffi::c_int,
        8 as std::ffi::c_int as std::ffi::c_uint,
    ) == 0
    {
        return blockSize;
    }
    HIST_add(
        ((*middleEvents).events).as_mut_ptr(),
        (blockStart as *const std::ffi::c_char)
            .offset((blockSize / 2 as std::ffi::c_int as usize) as isize)
            .offset(-((SEGMENT_SIZE / 2 as std::ffi::c_int) as isize))
            as *const std::ffi::c_void,
        SEGMENT_SIZE as usize,
    );
    (*middleEvents).nbEvents = SEGMENT_SIZE as usize;
    let distFromBegin = fpDistance(
        &mut (*fpstats).pastEvents,
        middleEvents,
        8 as std::ffi::c_int as std::ffi::c_uint,
    );
    let distFromEnd = fpDistance(
        &mut (*fpstats).newEvents,
        middleEvents,
        8 as std::ffi::c_int as std::ffi::c_uint,
    );
    let minDistance = (SEGMENT_SIZE * SEGMENT_SIZE / 3 as std::ffi::c_int) as u64;
    if abs64(distFromBegin as i64 - distFromEnd as i64) < minDistance {
        return (64 as std::ffi::c_int
            * ((1 as std::ffi::c_int) << 10)) as usize;
    }
    return (if distFromBegin > distFromEnd {
        32 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10)
    } else {
        96 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10)
    }) as usize;
}
pub const SEGMENT_SIZE: std::ffi::c_int = 512 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_splitBlock(
    mut blockStart: *const std::ffi::c_void,
    mut blockSize: usize,
    mut level: std::ffi::c_int,
    mut workspace: *mut std::ffi::c_void,
    mut wkspSize: usize,
) -> usize {
    if level == 0 as std::ffi::c_int {
        return ZSTD_splitBlock_fromBorders(blockStart, blockSize, workspace, wkspSize);
    }
    return ZSTD_splitBlock_byChunks(
        blockStart,
        blockSize,
        level - 1 as std::ffi::c_int,
        workspace,
        wkspSize,
    );
}
