// Generated macro for hide_sharp_behind_comment (function)
macro_rules! Depcrate_commenthide_sharp_behind_comment {
() => {
// Module: crate::comment
// Provides: {"hide_sharp_behind_comment"}
// Dependencies: {}
fn hide_sharp_behind_comment (s : & str) -> Cow < '_ , str > { let s_trimmed = s . trim () ; if s_trimmed . starts_with ("# ") || s_trimmed == "#" { Cow :: from (format ! ("{RUSTFMT_CUSTOM_COMMENT_PREFIX}{s}")) } else { Cow :: from (s) } }
};
}
