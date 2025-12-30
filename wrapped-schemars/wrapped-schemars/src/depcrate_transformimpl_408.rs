// Generated macro for impl_408 (impl)
macro_rules! Depcrate_transformimpl_408 {
() => {
// Module: crate::transform
// Provides: {"impl_408"}
// Dependencies: {}
impl Transform for ReplaceUnevaluatedProperties { fn transform (& mut self , schema : & mut Schema) { transform_subschemas (self , schema) ; let Some (up) = schema . remove ("unevaluatedProperties") else { return ; } ; schema . insert ("additionalProperties" . to_owned () , up) ; let mut gather_property_names = GatherPropertyNames :: default () ; gather_property_names . transform (schema) ; let property_names = gather_property_names . 0 ; if property_names . is_empty () { return ; } if let Some (properties) = schema . ensure_object () . entry ("properties") . or_insert (Map :: new () . into ()) . as_object_mut () { for name in property_names { properties . entry (name) . or_insert (true . into ()) ; } } } }
};
}
