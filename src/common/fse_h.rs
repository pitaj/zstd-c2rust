use std::mem::size_of;

pub const FSE_MAX_SYMBOL_VALUE: usize = 255;

pub type FSE_DTable = std::ffi::c_uint;

pub const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue: u32, tableLog: u32) -> usize {
    ((maxSymbolValue + 2) + (1_usize << tableLog))/2 + 
    size_of::<u64>()/size_of::<u32>() // additional 8 bytes for potential table overwrite
}
pub const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE(maxSymbolValue: u32, tableLog: u32) -> usize {
    size_of::<u32>() * FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog)
}

pub const fn FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    size_of<std::ffi::c_short>() * ((maxSymbolValue as usize) + 1) + (1_usize << maxTableLog) + 8
}
pub const fn FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    (FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog, maxSymbolValue) + size_of::<u32>() - 1) / size_of::<u32>()
}

pub const FSE_NCOUNTBOUND: usize = 512;
pub const fn FSE_BLOCKBOUND(size: usize) -> usize {
    size + (size >> 7) + 4 /* fse states */ + size_of::<usize>() /* bitContainer */
}
pub const fn FSE_COMPRESSBOUND(size: usize) -> usize {
    FSE_NCOUNTBOUND + FSE_BLOCKBOUND(size)
}

pub const fn FSE_DTABLE_SIZE_U32(maxTableLog: u32) -> usize {
    1 + (1_usize << maxTableLog)
}
pub const fn FSE_DTABLE_SIZE(maxTableLog: u32) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog) * size_of::<FSE_DTable>()
}

pub const fn FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog) + 1 + FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) + (FSE_MAX_SYMBOL_VALUE + 1) / 2 + 1
}
pub const fn FSE_DECOMPRESS_WKSP_SIZE(maxTableLog: u32, maxSymbolValue: u32) -> usize {
    FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog, maxSymbolValue) * size_of::<u32>()
}

pub const fn FSE_TABLESTEP(tableSize: usize) -> usize {
    (tableSize >> 1) + ((tableSize >> 3) + 3)
}
