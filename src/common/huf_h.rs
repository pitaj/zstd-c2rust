use std::mem::size_of;

pub const HUF_BLOCKSIZE_MAX: usize = 128 * 1024;

pub const HUF_WORKSPACE_SIZE: usize = (8_usize << 10) + 512;
pub const HUF_WORKSPACE_SIZE_U64: usize = HUF_WORKSPACE_SIZE / size_of::<u64>();

/* *** Constants *** */
const HUF_TABLELOG_MAX: u32 =      12;      /* max runtime value of tableLog (due to static allocation); can be modified up to HUF_TABLELOG_ABSOLUTEMAX */
const HUF_TABLELOG_DEFAULT: u32 =  11;      /* default tableLog value when none specified */
const HUF_SYMBOLVALUE_MAX: u32 =  255;

const HUF_TABLELOG_ABSOLUTEMAX: u32 =  12;  /* absolute limit of HUF_MAX_TABLELOG. Beyond that value, code does not work */
const _: () = assert!(HUF_TABLELOG_MAX <= HUF_TABLELOG_ABSOLUTEMAX);
