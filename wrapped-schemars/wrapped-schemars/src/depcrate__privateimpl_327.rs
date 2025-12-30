// Generated macro for impl_327 (impl)
macro_rules! Depcrate__privateimpl_327 {
() => {
// Module: crate::_private
// Provides: {"impl_327"}
// Dependencies: {}
impl Transform for AllowUnknownProperties { fn transform (& mut self , schema : & mut Schema) { if schema . get ("additionalProperties") . and_then (Value :: as_bool) == Some (false) { schema . remove ("additionalProperties") ; self . did_modify = true ; } if schema . get ("unevaluatedProperties") . and_then (Value :: as_bool) == Some (false) { schema . remove ("unevaluatedProperties") ; self . did_modify = true ; } transform_immediate_subschemas (self , schema) ; } }
};
}
