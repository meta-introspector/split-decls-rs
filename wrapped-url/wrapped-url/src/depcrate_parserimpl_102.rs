// Generated macro for impl_102 (impl)
macro_rules! Depcrate_parserimpl_102 {
() => {
// Module: crate::parser
// Provides: {"impl_102"}
// Dependencies: {}
impl Iterator for Input < '_ > { type Item = char ; fn next (& mut self) -> Option < char > { self . chars . by_ref () . find (| & c | ! ascii_tab_or_new_line (c)) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . chars . as_str () . len ())) } }
};
}
