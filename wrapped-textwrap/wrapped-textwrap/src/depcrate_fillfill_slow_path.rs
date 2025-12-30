// Generated macro for fill_slow_path (function)
macro_rules! Depcrate_fillfill_slow_path {
() => {
// Module: crate::fill
// Provides: {"fill_slow_path"}
// Dependencies: {}
# [doc = " Slow path for fill."] # [doc = ""] # [doc = " This is taken when `text` is longer than `options.width`."] pub (crate) fn fill_slow_path (text : & str , options : Options < '_ >) -> String { let mut result = String :: with_capacity (text . len ()) ; let line_ending_str = options . line_ending . as_str () ; for (i , line) in wrap (text , options) . iter () . enumerate () { if i > 0 { result . push_str (line_ending_str) ; } result . push_str (line) ; } result }
};
}
