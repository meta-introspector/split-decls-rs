// Generated macro for transform_immediate_subschemas (function)
macro_rules! Depcrate_transformtransform_immediate_subschemas {
() => {
// Module: crate::transform
// Provides: {"transform_immediate_subschemas"}
// Dependencies: {}
pub (crate) fn transform_immediate_subschemas < T : Transform + ? Sized > (t : & mut T , schema : & mut Schema ,) { for (key , value) in schema . as_object_mut () . into_iter () . flatten () { match key . as_str () { "if" | "then" | "else" => { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } "allOf" | "anyOf" | "oneOf" => { if let Some (array) = value . as_array_mut () { for value in array { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } } } _ => { } } } }
};
}
