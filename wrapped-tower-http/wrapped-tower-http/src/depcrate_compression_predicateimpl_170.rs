// Generated macro for impl_170 (impl)
macro_rules! Depcrate_compression_predicateimpl_170 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_170"}
// Dependencies: {}
impl Predicate for NotForContentType { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { if let Some (except) = & self . exception { if content_type (response) == except . as_str () { return true ; } } ! content_type (response) . starts_with (self . content_type . as_str ()) } }
};
}
