// Generated macro for AddNullable (struct)
macro_rules! Depcrate_transformAddNullable {
() => {
// Module: crate::transform
// Provides: {"AddNullable"}
// Dependencies: {}
# [doc = " Adds a `\"nullable\": true` property to schemas that allow `null` types."] # [doc = ""] # [doc = " This also applies to subschemas."] # [doc = ""] # [doc = " This is useful for dialects of JSON Schema (e.g. OpenAPI 3.0) that use `nullable` instead of"] # [doc = " explicit null types."] # [derive (Debug , Clone)] # [non_exhaustive] pub struct AddNullable { # [doc = " When set to `true` (the default), `\"null\"` will also be removed from the schemas `type`."] pub remove_null_type : bool , # [doc = " When set to `true` (the default), a schema that has a type only allowing `null` will also"] # [doc = " have the equivalent `\"const\": null` inserted."] pub add_const_null : bool , }
};
}
