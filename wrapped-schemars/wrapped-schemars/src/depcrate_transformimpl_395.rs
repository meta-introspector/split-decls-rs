// Generated macro for impl_395 (impl)
macro_rules! Depcrate_transformimpl_395 {
() => {
// Module: crate::transform
// Provides: {"impl_395"}
// Dependencies: {}
impl Transform for ReplaceBoolSchemas { fn transform (& mut self , schema : & mut Schema) { if let Some (obj) = schema . as_object_mut () { if self . skip_additional_properties { if let Some ((ap_key , ap_value)) = obj . remove_entry ("additionalProperties") { transform_subschemas (self , schema) ; schema . insert (ap_key , ap_value) ; return ; } } transform_subschemas (self , schema) ; } else { schema . ensure_object () ; } } }
};
}
