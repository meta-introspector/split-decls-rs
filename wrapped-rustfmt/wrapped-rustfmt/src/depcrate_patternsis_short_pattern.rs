// Generated macro for is_short_pattern (function)
macro_rules! Depcrate_patternsis_short_pattern {
() => {
// Module: crate::patterns
// Provides: {"is_short_pattern"}
// Dependencies: {}
# [doc = " Returns `true` if the given pattern is \"short\"."] # [doc = " A short pattern is defined by the following grammar:"] # [doc = ""] # [doc = " `[small, ntp]`:"] # [doc = "     - single token"] # [doc = "     - `&[single-line, ntp]`"] # [doc = ""] # [doc = " `[small]`:"] # [doc = "     - `[small, ntp]`"] # [doc = "     - unary tuple constructor `([small, ntp])`"] # [doc = "     - `&[small]`"] pub (crate) fn is_short_pattern (context : & RewriteContext < '_ > , pat : & ast :: Pat , pat_str : & str ,) -> bool { pat_str . len () <= 20 && ! pat_str . contains ('\n') && is_short_pattern_inner (context , pat) }
};
}
