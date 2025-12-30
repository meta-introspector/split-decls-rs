// Generated macro for impl_393 (impl)
macro_rules! Depcrate_transformimpl_393 {
() => {
// Module: crate::transform
// Provides: {"impl_393"}
// Dependencies: {}
impl < T > Transform for RecursiveTransform < T > where T : Transform , { fn transform (& mut self , schema : & mut Schema) { self . 0 . transform (schema) ; transform_subschemas (self , schema) ; } }
};
}
