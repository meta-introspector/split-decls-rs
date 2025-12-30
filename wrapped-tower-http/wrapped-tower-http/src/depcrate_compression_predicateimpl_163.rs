// Generated macro for impl_163 (impl)
macro_rules! Depcrate_compression_predicateimpl_163 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_163"}
// Dependencies: {}
impl Predicate for DefaultPredicate { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { self . 0 . should_compress (response) } }
};
}
