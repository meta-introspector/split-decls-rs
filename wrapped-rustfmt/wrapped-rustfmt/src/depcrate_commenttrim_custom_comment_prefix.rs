// Generated macro for trim_custom_comment_prefix (function)
macro_rules! Depcrate_commenttrim_custom_comment_prefix {
() => {
// Module: crate::comment
// Provides: {"trim_custom_comment_prefix"}
// Dependencies: {}
fn trim_custom_comment_prefix (s : & str) -> String { s . lines () . map (| line | { let left_trimmed = line . trim_start () ; if left_trimmed . starts_with (RUSTFMT_CUSTOM_COMMENT_PREFIX) { left_trimmed . trim_start_matches (RUSTFMT_CUSTOM_COMMENT_PREFIX) } else { line } }) . collect :: < Vec < _ > > () . join ("\n") }
};
}
