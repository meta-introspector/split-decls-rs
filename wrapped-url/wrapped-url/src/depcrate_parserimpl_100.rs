// Generated macro for impl_100 (impl)
macro_rules! Depcrate_parserimpl_100 {
() => {
// Module: crate::parser
// Provides: {"impl_100"}
// Dependencies: {}
impl Pattern for & str { fn split_prefix (self , input : & mut Input) -> bool { for c in self . chars () { if input . next () != Some (c) { return false ; } } true } }
};
}
