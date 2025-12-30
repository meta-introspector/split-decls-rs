// Generated macro for transform_subschemas (function)
macro_rules! Depcrate_transformtransform_subschemas {
() => {
// Module: crate::transform
// Provides: {"transform_subschemas"}
// Dependencies: {}
# [doc = " Applies the given [`Transform`] to all direct subschemas of the [`Schema`]."] pub fn transform_subschemas < T : Transform + ? Sized > (t : & mut T , schema : & mut Schema) { for (key , value) in schema . as_object_mut () . into_iter () . flatten () { match key . as_str () { "not" | "if" | "then" | "else" | "contains" | "additionalProperties" | "propertyNames" | "additionalItems" => { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } "allOf" | "anyOf" | "oneOf" | "prefixItems" => { if let Some (array) = value . as_array_mut () { for value in array { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } } } "items" => { if let Some (array) = value . as_array_mut () { for value in array { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } } else if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } "properties" | "patternProperties" | "$defs" | "definitions" => { if let Some (obj) = value . as_object_mut () { for value in obj . values_mut () { if let Ok (subschema) = value . try_into () { t . transform (subschema) ; } } } } _ => { } } } }
};
}
