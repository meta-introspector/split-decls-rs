// Generated macro for find_line (function)
macro_rules! Depcrate_fluent_periodfind_line {
() => {
// Module: crate::fluent_period
// Provides: {"find_line"}
// Dependencies: {}
# [doc = " Evil cursed bad hack. Requires that `value` be a substr (in memory) of `contents`."] fn find_line (haystack : & str , needle : & str) -> usize { for (ll , line) in haystack . lines () . enumerate () { if line . as_ptr () > needle . as_ptr () { return ll ; } } 1 }
};
}
