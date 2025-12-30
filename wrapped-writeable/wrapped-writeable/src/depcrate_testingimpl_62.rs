// Generated macro for impl_62 (impl)
macro_rules! Depcrate_testingimpl_62 {
() => {
// Module: crate::testing
// Provides: {"impl_62"}
// Dependencies: {}
impl TestWriter { pub (crate) fn finish (mut self) -> (String , Vec < (usize , usize , Part) >) { self . parts . sort_unstable_by_key (| (begin , end , _) | (* begin , end . wrapping_neg ())) ; (self . string , self . parts) } }
};
}
