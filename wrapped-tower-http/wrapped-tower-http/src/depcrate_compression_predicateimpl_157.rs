// Generated macro for impl_157 (impl)
macro_rules! Depcrate_compression_predicateimpl_157 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_157"}
// Dependencies: {}
impl < T > Predicate for Option < T > where T : Predicate , { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { self . as_ref () . map (| inner | inner . should_compress (response)) . unwrap_or (true) } }
};
}
