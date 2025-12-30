// Generated macro for impl_101 (impl)
macro_rules! Depcrate_parserimpl_101 {
() => {
// Module: crate::parser
// Provides: {"impl_101"}
// Dependencies: {}
impl < F : FnMut (char) -> bool > Pattern for F { fn split_prefix (self , input : & mut Input) -> bool { input . next () . map_or (false , self) } }
};
}
