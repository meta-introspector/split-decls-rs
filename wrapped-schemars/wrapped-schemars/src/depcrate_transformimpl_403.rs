// Generated macro for impl_403 (impl)
macro_rules! Depcrate_transformimpl_403 {
() => {
// Module: crate::transform
// Provides: {"impl_403"}
// Dependencies: {}
impl Transform for ReplacePrefixItems { fn transform (& mut self , schema : & mut Schema) { transform_subschemas (self , schema) ; if let Some (prefix_items) = schema . remove ("prefixItems") { let previous_items = schema . insert ("items" . to_owned () , prefix_items) ; if let Some (previous_items) = previous_items { schema . insert ("additionalItems" . to_owned () , previous_items) ; } } } }
};
}
