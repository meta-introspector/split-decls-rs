// Generated macro for unified_diff (function)
macro_rules! Depcrate_udiffunified_diff {
() => {
// Module: crate::udiff
// Provides: {"unified_diff"}
// Dependencies: {}
# [doc = " Quick way to get a unified diff as string."] # [doc = ""] # [doc = " `n` configures [`UnifiedDiff::context_radius`] and"] # [doc = " `header` configures [`UnifiedDiff::header`] when not `None`."] pub fn unified_diff (alg : Algorithm , old : & str , new : & str , n : usize , header : Option < (& str , & str) > ,) -> String { TextDiff :: configure () . algorithm (alg) . diff_lines (old , new) . unified_diff () . context_radius (n) . header_opt (header) . to_string () }
};
}
