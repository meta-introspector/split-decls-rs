// Generated macro for impl_156 (impl)
macro_rules! Depcrate_compression_predicateimpl_156 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_156"}
// Dependencies: {}
impl < F > Predicate for F where F : Fn (StatusCode , Version , & HeaderMap , & Extensions) -> bool + Clone , { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { let status = response . status () ; let version = response . version () ; let headers = response . headers () ; let extensions = response . extensions () ; self (status , version , headers , extensions) } }
};
}
