use std::ffi::c_void;

pub const MEM_32bits: bool = usize::BITS == 32;
pub const MEM_64bits: bool = usize::BITS == 64;

pub const MEM_isLittleEndian: bool = cfg!(target_endian = "little");

/*=== Native unaligned read/write ===*/

#[inline(always)]
pub unsafe fn MEM_read16(memPtr: *const c_void) -> u16 {
    memPtr.cast().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_read32(memPtr: *const c_void) -> u32 {
    memPtr.cast().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_read64(memPtr: *const c_void) -> u64 {
    memPtr.cast().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_readST(memPtr: *const c_void) -> usize {
    memPtr.cast().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_write16(memPtr: *mut c_void, value: u16) {
    memPtr.cast().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_write32(memPtr: *mut c_void, value: u32) {
    memPtr.cast().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_write64(memPtr: *mut c_void, value: u64) {
    memPtr.cast().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_writeST(memPtr: *mut c_void, value: usize) {
    memPtr.cast().write_unaligned(value);
}

/*=== Little endian unaligned read/write ===*/

#[cfg(target_endian = "little")]
mod le_on_le {
    use super::*;

    pub use MEM_read16 as MEM_readLE16;
    pub use MEM_read32 as MEM_readLE32;
    pub use MEM_read64 as MEM_readLE64;
    pub use MEM_readST as MEM_readLEST;

    pub use MEM_write16 as MEM_writeLE16;
    pub use MEM_write32 as MEM_writeLE32;
    pub use MEM_write64 as MEM_writeLE64;
    pub use MEM_writeST as MEM_writeLEST;
}
#[cfg(target_endian = "little")]
pub use le_on_le::*;

#[cfg(target_endian = "big")]
mod le_on_be {
    use super::*;

    #[inline(always)]
    pub unsafe fn MEM_readLE16(memPtr: *const c_void) -> u16 {
        MEM_read16(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readLE32(memPtr: *const c_void) -> u32 {
        MEM_read32(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readLE64(memPtr: *const c_void) -> u64 {
        MEM_read64(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readLEST(memPtr: *const c_void) -> usize {
        MEM_readST(memPtr).swap_bytes()
    }

    #[inline(always)]
    pub unsafe fn MEM_writeLE16(memPtr: *mut c_void, value: u16) {
        MEM_write16(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeLE32(memPtr: *mut c_void, value: u32) {
        MEM_write32(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeLE64(memPtr: *mut c_void, value: u64) {
        MEM_write64(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeLEST(memPtr: *mut c_void, value: usize) {
        MEM_writeST(memPtr, value.swap_bytes())
    }
}
#[cfg(target_endian = "big")]
pub use le_on_be::*;

/*=== Big endian unaligned read/write ===*/

#[cfg(target_endian = "little")]
mod be_on_le {
    use super::*;

    #[inline(always)]
    pub unsafe fn MEM_readBE16(memPtr: *const c_void) -> u16 {
        MEM_read16(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readBE32(memPtr: *const c_void) -> u32 {
        MEM_read32(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readBE64(memPtr: *const c_void) -> u64 {
        MEM_read64(memPtr).swap_bytes()
    }
    #[inline(always)]
    pub unsafe fn MEM_readBEST(memPtr: *const c_void) -> usize {
        MEM_readST(memPtr).swap_bytes()
    }

    #[inline(always)]
    pub unsafe fn MEM_writeBE16(memPtr: *mut c_void, value: u16) {
        MEM_write16(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeBE32(memPtr: *mut c_void, value: u32) {
        MEM_write32(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeBE64(memPtr: *mut c_void, value: u64) {
        MEM_write64(memPtr, value.swap_bytes())
    }
    #[inline(always)]
    pub unsafe fn MEM_writeBEST(memPtr: *mut c_void, value: usize) {
        MEM_writeST(memPtr, value.swap_bytes())
    }
}
#[cfg(target_endian = "little")]
pub use be_on_le::*;

#[cfg(target_endian = "big")]
mod be_on_be {
    use super::*;

    pub use MEM_read16 as MEM_readBE16;
    pub use MEM_read32 as MEM_readBE32;
    pub use MEM_read64 as MEM_readBE64;
    pub use MEM_readST as MEM_readBEST;

    pub use MEM_write16 as MEM_writeBE16;
    pub use MEM_write32 as MEM_writeBE32;
    pub use MEM_write64 as MEM_writeBE64;
    pub use MEM_writeST as MEM_writeBEST;
}
#[cfg(target_endian = "big")]
pub use be_on_be::*;

/*=== Byteswap ===*/

pub fn MEM_swap32(input: u32) -> u32 {
    input.swap_bytes()
}
pub fn MEM_swap64(input: u64) -> u64 {
    input.swap_bytes()
}
pub fn MEM_swapST(input: usize) -> usize {
    input.swap_bytes()
}
