/*-*********************************************
 *  Error codes list
 *-*********************************************
 *  Error codes _values_ are pinned down since v1.3.1 only.
 *  Therefore, don't rely on values if you may link to any version < v1.3.1.
 *
 *  Only values < 100 are considered stable.
 *
 *  note 1 : this API shall be used with static linking only.
 *           dynamic linking is not yet officially supported.
 *  note 2 : Prefer relying on the enum than on its value whenever possible
 *           This is the only supported way to use the error list < v1.3.1
 *  note 3 : ZSTD_isError() is always correct, whatever the library version.
 **********************************************/
pub type ZSTD_ErrorCode = usize;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: ZSTD_ErrorCode = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;

#[inline(always)]
pub const fn ERROR(code: ZSTD_ErrorCode) -> usize {
    -(code as isize) as usize
}

#[inline]
pub const fn ERR_isError(code: ZSTD_ErrorCode) -> bool {
    code > ERROR(ZSTD_error_maxCode)
}

#[inline]
pub const fn ERR_getErrorCode(code: usize) -> ZSTD_ErrorCode {
    if ERR_isError(code) {
        return ZSTD_error_no_error;
    }
    return 0_usize.wrapping_sub(code) as ZSTD_ErrorCode;
}

#[inline]
pub const fn ERR_getErrorName(code: usize) -> &'static str {
    return ERR_getErrorString(ERR_getErrorCode(code));
}

#[inline]
pub const fn ERR_getErrorString(code: ZSTD_ErrorCode) -> &'static str {
    match code {
        0 => "No error detected",
        1 => "Error (generic)",
        10 => "Unknown frame descriptor",
        12 => "Version not supported",
        14 => "Unsupported frame parameter",
        16 => "Frame requires too much memory for decoding",
        20 => "Data corruption detected",
        22 => "Restored data doesn't match checksum",
        24 => "Header of Literals' block doesn't respect format specification",
        40 => "Unsupported parameter",
        41 => "Unsupported combination of parameters",
        42 => "Parameter is out of bound",
        62 => "Context should be init first",
        64 => "Allocation error : not enough memory",
        66 => "workSpace buffer is not large enough",
        60 => "Operation not authorized at current processing stage",
        44 => "tableLog requires too much memory : unsupported",
        46 => "Unsupported max Symbol Value : too large",
        48 => "Specified maxSymbolValue is too small",
        49 => "This mode cannot generate an uncompressed block",
        50 => "pledged buffer stability condition is not respected",
        30 => "Dictionary is corrupted",
        32 => "Dictionary mismatch",
        34 => "Cannot create Dictionary from provided samples",
        70 => "Destination buffer is too small",
        72 => "Src size is incorrect",
        74 => "Operation on NULL destination buffer",
        80 => "Operation made no progress over multiple calls, due to output buffer being full",
        82 => "Operation made no progress over multiple calls, due to input being empty",
        100 => "Frame index is too large",
        102 => "An I/O error occurred when reading/seeking",
        104 => "Destination buffer is wrong",
        105 => "Source buffer is wrong",
        106 => "Block-level external sequence producer returned an error code",
        107 => "External sequences are not valid",
        120 | _ => "Unspecified error code",
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __RETURN_ERROR_IF {
    ($cond:expr, $error:expr $(, $reason:literal $(, $rest:expr)*)?) => {
        if $cond {
            return $crate::common::error::ERROR($error);
        }
    }
}
pub use crate::__RETURN_ERROR_IF as RETURN_ERROR_IF;

#[doc(hidden)]
#[macro_export]
macro_rules! __FORWARD_IF_ERROR {
    ($err:expr $(, $reason:literal)?) => {
        if $crate::common::error::ERR_isError($err) {
            return $err;
        }
    }
}
pub use crate::__FORWARD_IF_ERROR as FORWARD_IF_ERROR;
