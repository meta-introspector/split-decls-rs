// Generated macro for impl_410 (impl)
macro_rules! Depcrate_transformimpl_410 {
() => {
// Module: crate::transform
// Provides: {"impl_410"}
// Dependencies: {}
impl Transform for GatherPropertyNames { fn transform (& mut self , schema : & mut Schema) { self . 0 . extend (schema . as_object () . iter () . filter_map (| o | o . get ("properties")) . filter_map (Value :: as_object) . flat_map (Map :: keys) . cloned () ,) ; transform_immediate_subschemas (self , schema) ; } }
};
}
