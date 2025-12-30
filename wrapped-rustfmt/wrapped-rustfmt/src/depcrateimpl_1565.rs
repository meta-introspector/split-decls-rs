// Generated macro for impl_1565 (impl)
macro_rules! Depcrateimpl_1565 {
() => {
// Module: crate
// Provides: {"impl_1565"}
// Dependencies: {}
impl FormattedSnippet { # [doc = " In case the snippet needed to be wrapped in a function, this shifts down the ranges of"] # [doc = " non-formatted code."] fn unwrap_code_block (& mut self) { self . non_formatted_ranges . iter_mut () . for_each (| (low , high) | { * low -= 1 ; * high -= 1 ; }) ; } # [doc = " Returns `true` if the line n did not get formatted."] fn is_line_non_formatted (& self , n : usize) -> bool { self . non_formatted_ranges . iter () . any (| (low , high) | * low <= n && n <= * high) } }
};
}
