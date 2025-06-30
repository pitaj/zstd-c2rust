use std::ffi::c_void;

pub const MEM_32bits: bool = usize::BITS == 32;
pub const MEM_64bits: bool = usize::BITS == 64;

pub const MEM_isLittleEndian: bool = cfg!(target_endian = "little");

/*=== Native unaligned read/write ===*/

#[inline(always)]
pub unsafe fn MEM_read16(memPtr: *const c_void) -> u16 {
    memPtr.cast::<u16>().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_read32(memPtr: *const c_void) -> u32 {
    memPtr.cast::<u32>().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_read64(memPtr: *const c_void) -> u64 {
    memPtr.cast::<u64>().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_readST(memPtr: *const c_void) -> usize {
    memPtr.cast::<usize>().read_unaligned()
}
#[inline(always)]
pub unsafe fn MEM_write16(memPtr: *mut c_void, value: u16) {
    memPtr.cast::<u16>().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_write32(memPtr: *mut c_void, value: u32) {
    memPtr.cast::<u32>().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_write64(memPtr: *mut c_void, value: u64) {
    memPtr.cast::<u64>().write_unaligned(value);
}
#[inline(always)]
pub unsafe fn MEM_writeST(memPtr: *mut c_void, value: usize) {
    memPtr.cast::<usize>().write_unaligned(value);
}

/*=== Little endian unaligned read/write ===*/

mod le_on_le {
    use std::ffi::c_void;

    pub use super::MEM_read16 as MEM_readLE16;
    pub use super::MEM_read32 as MEM_readLE32;
    pub use super::MEM_read64 as MEM_readLE64;
    pub use super::MEM_readST as MEM_readLEST;

    #[inline]
    pub unsafe fn MEM_readLE24(memPtr: *const c_void) -> u32 {
        u32::from(MEM_readLE16(memPtr)) + 
            (u32::from(*memPtr.cast::<u8>().offset(2)) << 16)
    }

    pub use super::MEM_write16 as MEM_writeLE16;
    pub use super::MEM_write32 as MEM_writeLE32;
    pub use super::MEM_write64 as MEM_writeLE64;
    pub use super::MEM_writeST as MEM_writeLEST;

    #[inline]
    pub unsafe fn MEM_writeLE24(memPtr: *mut c_void, value: u32) {
        MEM_writeLE16(memPtr, value as u16);
        *memPtr.cast::<u8>().offset(2) = (value >> 16) as u8;
    }
}
#[cfg(target_endian = "little")]
pub use le_on_le::*;

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

    #[inline]
    pub unsafe fn MEM_readLE24(memPtr: *const c_void) -> u32 {
        todo!()
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

    #[inline]
    pub unsafe fn MEM_writeLE24(memPtr: *mut c_void, value: u32) {
        todo!()
    }
}
#[cfg(target_endian = "big")]
pub use le_on_be::*;

/*=== Big endian unaligned read/write ===*/

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

mod be_on_be {
    pub use super::MEM_read16 as MEM_readBE16;
    pub use super::MEM_read32 as MEM_readBE32;
    pub use super::MEM_read64 as MEM_readBE64;
    pub use super::MEM_readST as MEM_readBEST;

    pub use super::MEM_write16 as MEM_writeBE16;
    pub use super::MEM_write32 as MEM_writeBE32;
    pub use super::MEM_write64 as MEM_writeBE64;
    pub use super::MEM_writeST as MEM_writeBEST;
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
