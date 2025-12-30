// Generated macro for impl_413 (impl)
macro_rules! Depcrate_transformimpl_413 {
() => {
// Module: crate::transform
// Provides: {"impl_413"}
// Dependencies: {}
impl Transform for RestrictFormats { fn transform (& mut self , schema : & mut Schema) { let mut implementation = RestrictFormatsImpl { infer_from_meta_schema : self . infer_from_meta_schema , inferred_formats : None , allowed_formats : & self . allowed_formats , } ; implementation . transform (schema) ; } }
};
}
