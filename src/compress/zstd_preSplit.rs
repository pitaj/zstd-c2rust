use std::mem::{align_of, size_of};
use std::ffi::c_void;

use crate::zstd_h::DEBUGLOG;
use crate::compress::hist::*;
use crate::common::mem::*;
use crate::common::zstd_internal_h::*;

// ==== from zstd_preSplit.h ====
pub const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;
// ==== end  zstd_preSplit.h ====

const THRESHOLD_PENALTY_RATE: u64 = 16;
const THRESHOLD_BASE: i64 = THRESHOLD_PENALTY_RATE as i64 - 2;
const THRESHOLD_PENALTY: i32 = 3;

const HASHLENGTH: usize = 2;
const HASHLOG_MAX: u32 = 10;
const HASHTABLESIZE: usize = 1_usize << HASHLOG_MAX;
const HASHMASK: usize = HASHTABLESIZE - 1;
const KNUTH: u32 = 0x9e3779b9;

/* for hashLog > 8, hash 2 bytes.
 * for hashLog == 8, just take the byte, no hashing.
 * The speed of this method relies on compile-time constant propagation */
#[inline(always)]
unsafe fn hash2(
    mut p: *const c_void,
    mut hashLog: u32,
) -> u32 {
    debug_assert!(hashLog >= 8);
    if hashLog == 8 {
        return *(p as *const u8).offset(0) as u32;
    }
    return (MEM_read16(p) as u32).wrapping_mul(KNUTH)
        >> 32_u32.wrapping_sub(hashLog);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fingerprint {
    pub events: [u32; HASHTABLESIZE],
    pub nbEvents: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}

unsafe fn initStats(mut fpstats: *mut FPStats) {
    libc::memset(
        fpstats as *mut c_void,
        0,
        size_of::<FPStats>() as usize,
    );
}

#[inline(always)]
unsafe fn addEvents_generic(
    mut fp: *mut Fingerprint,
    mut src: *const c_void,
    mut srcSize: usize,
    mut samplingRate: usize,
    mut hashLog: u32,
) {
    let mut p = src as *const std::ffi::c_char;
    let mut limit = srcSize + 1 - HASHLENGTH;
    let mut n: usize = 0;
    while n < limit {
        (*fp).events[hash2(p.add(n).cast(), hashLog) as usize] = 
            ((*fp).events[hash2(p.add(n).cast(), hashLog) as usize]).wrapping_add(1);
        n = n.wrapping_add(samplingRate);
    }
    (*fp).nbEvents = ((*fp).nbEvents).wrapping_add(limit / samplingRate);
}

#[inline(always)]
unsafe fn recordFingerprint_generic(
    mut fp: *mut Fingerprint,
    mut src: *const c_void,
    mut srcSize: usize,
    mut samplingRate: usize,
    mut hashLog: u32,
) {
    libc::memset(
        fp as *mut c_void,
        0,
        size_of::<u32>() * 1_usize << hashLog,
    );
    (*fp).nbEvents = 0;
    addEvents_generic(fp, src, srcSize, samplingRate, hashLog);
}

pub type RecordEvents_f = unsafe fn(*mut Fingerprint, *const c_void, usize) -> ();

macro_rules! ZSTD_GEN_RECORD_FINGERPRINT {
    ($name:ident, $rate:literal, $hSize:literal) => {
        #[inline]
        unsafe fn $name(fp: *mut Fingerprint, src: *const c_void, srcSize: usize) {
            recordFingerprint_generic(fp, src, srcSize, $rate, $hSize);
        }
    }
}

ZSTD_GEN_RECORD_FINGERPRINT!(ZSTD_recordFingerprint_1, 1, 10);
ZSTD_GEN_RECORD_FINGERPRINT!(ZSTD_recordFingerprint_5, 5, 10);
ZSTD_GEN_RECORD_FINGERPRINT!(ZSTD_recordFingerprint_11, 11, 9);
ZSTD_GEN_RECORD_FINGERPRINT!(ZSTD_recordFingerprint_43, 43, 8);


unsafe fn abs64(mut s64: i64) -> u64 {
    s64.unsigned_abs()
}

unsafe fn fpDistance(
    mut fp1: *const Fingerprint,
    mut fp2: *const Fingerprint,
    mut hashLog: u32,
) -> u64 {
    let mut distance: u64 = 0;
    debug_assert!(hashLog <= HASHLOG_MAX);
    for n in 0..(1_usize << hashLog) {
        distance = distance.wrapping_add(
            i64::abs_diff(
            i64::from((*fp1).events[n]) * ((*fp2).nbEvents as i64),
                i64::from((*fp2).events[n]) * ((*fp1).nbEvents as i64)
            )
        );
    }
    distance
}

/* Compare newEvents with pastEvents
 * return 1 when considered "too different"
 */
unsafe fn compareFingerprints(
    mut ref_0: *const Fingerprint,
    mut newfp: *const Fingerprint,
    mut penalty: i32,
    mut hashLog: u32,
) -> bool {
    debug_assert!((*ref_0).nbEvents > 0);
    debug_assert!((*newfp).nbEvents > 0);

    let mut p50 = ((*ref_0).nbEvents as u64) * ((*newfp).nbEvents as u64);
    let mut deviation = fpDistance(ref_0, newfp, hashLog);
    let mut threshold = p50 * ((THRESHOLD_BASE + i64::from(penalty)) as u64)
        / THRESHOLD_PENALTY_RATE;
    deviation >= threshold
}

unsafe fn mergeEvents(
    mut acc: *mut Fingerprint,
    mut newfp: *const Fingerprint,
) {
    for n in 0..HASHTABLESIZE {
        (*acc).events[n] = ((*acc).events[n]).wrapping_add((*newfp).events[n]);
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_add((*newfp).nbEvents);
}

unsafe fn flushEvents(mut fpstats: *mut FPStats) {
    for n in 0..HASHTABLESIZE {
        (*fpstats).pastEvents.events[n] = (*fpstats).newEvents.events[n];
    }
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    libc::memset(
        &mut (*fpstats).newEvents as *mut Fingerprint as *mut c_void,
        0,
        size_of::<Fingerprint>(),
    );
}

unsafe fn removeEvents(
    mut acc: *mut Fingerprint,
    mut slice: *const Fingerprint,
) {
    for n in 0..HASHTABLESIZE {
        (*acc).events[n] = ((*acc).events[n]).wrapping_sub((*slice).events[n]);
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_sub((*slice).nbEvents);
}

pub const CHUNKSIZE: usize = 8_usize << 10;
unsafe fn ZSTD_splitBlock_byChunks(
    mut blockStart: *const c_void,
    mut blockSize: usize,
    mut level: i32,
    mut workspace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    const records_fs: [RecordEvents_f; 4] = [
        ZSTD_recordFingerprint_43,
        ZSTD_recordFingerprint_11,
        ZSTD_recordFingerprint_5,
        ZSTD_recordFingerprint_1,
    ];
    const hashParams: [u32; 4] = [
        8,
        9,
        10,
        10,
    ];
    let record_f: RecordEvents_f = records_fs[level as usize];
    let fpstats = workspace as *mut FPStats;
    let mut p = blockStart as *const std::ffi::c_char;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos: usize = 0;
    debug_assert!(blockSize == (128_usize << 10));
    debug_assert!(!workspace.is_null());
    debug_assert!(workspace.is_aligned_to(align_of::<FPStats>()));
    const _: () = assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>());
    debug_assert!(wkspSize >= size_of::<FPStats>());

    initStats(fpstats);
    record_f(&mut (*fpstats).pastEvents, p as *const c_void, CHUNKSIZE);
    pos = CHUNKSIZE;
    while pos <= blockSize.wrapping_sub(CHUNKSIZE) {
        record_f(
            &mut (*fpstats).newEvents,
            p.add(pos) as *const c_void,
            CHUNKSIZE,
        );
        if compareFingerprints(
            &mut (*fpstats).pastEvents,
            &mut (*fpstats).newEvents,
            penalty,
            hashParams[level as usize],
        )
        {
            return pos
        } else {
            mergeEvents(&mut (*fpstats).pastEvents, &mut (*fpstats).newEvents);
            if penalty > 0 {
                penalty -= 1;
            }
        }
        pos = pos.wrapping_add(CHUNKSIZE as usize);
    }
    debug_assert!(pos == blockSize);
    blockSize
}

/* ZSTD_splitBlock_fromBorders(): very fast strategy :
 * compare fingerprint from beginning and end of the block,
 * derive from their difference if it's preferable to split in the middle,
 * repeat the process a second time, for finer grained decision.
 * 3 times did not brought improvements, so I stopped at 2.
 * Benefits are good enough for a cheap heuristic.
 * More accurate splitting saves more, but speed impact is also more perceptible.
 * For better accuracy, use more elaborate variant *_byChunks.
 */
unsafe fn ZSTD_splitBlock_fromBorders(
    mut blockStart: *const c_void,
    mut blockSize: usize,
    mut workspace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    const SEGMENT_SIZE: usize = 512;

    let fpstats = workspace as *mut FPStats;
    let mut middleEvents = workspace.byte_add(512_usize * size_of::<u32>()) as *mut Fingerprint;
    debug_assert!(blockSize == (128 << 10));
    debug_assert!(!workspace.is_null());
    debug_assert!(workspace.is_aligned_to(align_of::<FPStats>()));
    const _: () = assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>());
    debug_assert!(wkspSize >= size_of::<FPStats>());

    initStats(fpstats);
    HIST_add(
        ((*fpstats).pastEvents.events).as_mut_ptr(),
        blockStart,
        SEGMENT_SIZE,
    );
    HIST_add(
        ((*fpstats).newEvents.events).as_mut_ptr(),
        blockStart
            .byte_add(blockSize)
            .byte_sub(SEGMENT_SIZE) as *const c_void,
        SEGMENT_SIZE,
    );
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE;
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    if !compareFingerprints(
        &mut (*fpstats).pastEvents,
        &mut (*fpstats).newEvents,
        0,
        8,
    )
    {
        return blockSize;
    }

    HIST_add(
        ((*middleEvents).events).as_mut_ptr(),
        blockStart
            .byte_add(blockSize / 2)
            .byte_sub(SEGMENT_SIZE / 2)
            as *const c_void,
        SEGMENT_SIZE,
    );
    (*middleEvents).nbEvents = SEGMENT_SIZE;
    let distFromBegin = fpDistance(
        &mut (*fpstats).pastEvents,
        middleEvents,
        8,
    );
    let distFromEnd = fpDistance(
        &mut (*fpstats).newEvents,
        middleEvents,
        8,
    );
    let minDistance = (SEGMENT_SIZE * SEGMENT_SIZE / 3) as u64;
    if i64::abs_diff(distFromBegin as i64, distFromEnd as i64) < minDistance {
        return KB(64);
    }
    if distFromBegin > distFromEnd {
        KB(32)
    } else {
        KB(96)
    }
}

/* ZSTD_splitBlock():
 * @level must be a value between 0 and 4.
 *        higher levels spend more energy to detect block boundaries.
 * @workspace must be aligned for size_t.
 * @wkspSize must be at least >= ZSTD_SLIPBLOCK_WORKSPACESIZE
 * note:
 * For the time being, this function only accepts full 128 KB blocks.
 * Therefore, @blockSize must be == 128 KB.
 * While this could be extended to smaller sizes in the future,
 * it is not yet clear if this would be useful. TBD.
 */
pub unsafe fn ZSTD_splitBlock(
    mut blockStart: *const c_void,
    mut blockSize: usize,
    mut level: i32,
    mut workspace: *mut c_void,
    mut wkspSize: usize,
) -> usize {
    DEBUGLOG!(6, "ZSTD_splitBlock (level=%i)", level);
    debug_assert!(0<=level && level<=4);

    if level == 0 {
        ZSTD_splitBlock_fromBorders(blockStart, blockSize, workspace, wkspSize)
    } else {
        ZSTD_splitBlock_byChunks(
            blockStart,
            blockSize,
            level - 1,
            workspace,
            wkspSize,
        )
    }
}
