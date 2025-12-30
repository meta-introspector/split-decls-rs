// Generated macro for impl_416 (impl)
macro_rules! Depcrate_transformimpl_416 {
() => {
// Module: crate::transform
// Provides: {"impl_416"}
// Dependencies: {}
impl Transform for RestrictFormatsImpl < '_ > { fn transform (& mut self , schema : & mut Schema) { let Some (obj) = schema . as_object_mut () else { return ; } ; let previous_inferred_formats = self . inferred_formats ; if self . infer_from_meta_schema && obj . contains_key ("$schema") { self . inferred_formats = match obj . get ("$schema") . and_then (Value :: as_str) . unwrap_or_default () { meta_schemas :: DRAFT07 => Some (& DEFINED_FORMATS [2 ..]) , meta_schemas :: DRAFT2019_09 | meta_schemas :: DRAFT2020_12 => Some (DEFINED_FORMATS) , _ => { return ; } } ; } if let Some (format) = obj . get ("format") . and_then (Value :: as_str) { if ! self . allowed_formats . contains (format) && ! self . inferred_formats . is_some_and (| formats | formats . contains (& format)) { obj . remove ("format") ; } } transform_subschemas (self , schema) ; self . inferred_formats = previous_inferred_formats ; } }
};
}
