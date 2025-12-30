// Generated macro for minify_val_fcs (function)
macro_rules! Depcrate_encoding_frame_headerminify_val_fcs {
() => {
// Module: crate::encoding::frame_header
// Provides: {"minify_val_fcs"}
// Dependencies: {}
# [doc = " Identical to [`minify_val`], but it implements the following edge case:"] # [doc = ""] # [doc = " > When FCS_Field_Size is 1, 4 or 8 bytes, the value is read directly. When FCS_Field_Size is 2, the offset of 256 is added."] # [doc = ""] # [doc = " https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_content_size"] fn minify_val_fcs (val : u64) -> Vec < u8 > { let new_size = find_min_size (val) ; let mut val = val ; if new_size == 2 { val -= 256 ; } val . to_le_bytes () [0 .. new_size] . to_vec () }
};
}
