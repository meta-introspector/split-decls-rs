// Generated macro for last_line_offsetted (function)
macro_rules! Depcrate_exprlast_line_offsetted {
() => {
// Module: crate::expr
// Provides: {"last_line_offsetted"}
// Dependencies: {}
# [doc = " Returns `true` if the last line of pat_str has leading whitespace and it is wider than the"] # [doc = " shape's indent."] fn last_line_offsetted (start_column : usize , pat_str : & str) -> bool { let mut leading_whitespaces = 0 ; for c in pat_str . chars () . rev () { match c { '\n' => break , _ if c . is_whitespace () => leading_whitespaces += 1 , _ => leading_whitespaces = 0 , } } leading_whitespaces > start_column }
};
}
