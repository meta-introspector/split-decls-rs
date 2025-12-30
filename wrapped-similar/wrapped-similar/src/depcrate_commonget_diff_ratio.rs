// Generated macro for get_diff_ratio (function)
macro_rules! Depcrate_commonget_diff_ratio {
() => {
// Module: crate::common
// Provides: {"get_diff_ratio"}
// Dependencies: {}
# [doc = " Return a measure of similarity in the range `0..=1`."] # [doc = ""] # [doc = " A ratio of `1.0` means the two sequences are a complete match, a"] # [doc = " ratio of `0.0` would indicate completely distinct sequences.  The input"] # [doc = " is the sequence of diff operations and the length of the old and new"] # [doc = " sequence."] pub fn get_diff_ratio (ops : & [DiffOp] , old_len : usize , new_len : usize) -> f32 { let matches = ops . iter () . map (| op | { if let DiffOp :: Equal { len , .. } = * op { len } else { 0 } }) . sum :: < usize > () ; let len = old_len + new_len ; if len == 0 { 1.0 } else { 2.0 * matches as f32 / len as f32 } }
};
}
