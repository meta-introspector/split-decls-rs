// Generated macro for impl_159 (impl)
macro_rules! Depcrate_compression_predicateimpl_159 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_159"}
// Dependencies: {}
impl < Lhs , Rhs > Predicate for And < Lhs , Rhs > where Lhs : Predicate , Rhs : Predicate , { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { self . lhs . should_compress (response) && self . rhs . should_compress (response) } }
};
}
