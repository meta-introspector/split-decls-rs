// Generated macro for impl_103 (impl)
macro_rules! Depcrate_data_runtimeimpl_103 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_103"}
// Dependencies: {}
impl PathRuntime { fn new (path_prefix : & str) -> Self { Self { path_prefix : path_prefix . to_owned () , count : 0 , } } fn is (& self , path_prefix : & str) -> bool { self . path_prefix == path_prefix } fn next (& mut self) -> usize { self . count += 1 ; self . count } fn count (& self) -> usize { self . count } }
};
}
