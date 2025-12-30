// Generated macro for ReplaceBoolSchemas (struct)
macro_rules! Depcrate_transformReplaceBoolSchemas {
() => {
// Module: crate::transform
// Provides: {"ReplaceBoolSchemas"}
// Dependencies: {}
# [doc = " Replaces boolean JSON Schemas with equivalent object schemas."] # [doc = ""] # [doc = " This also applies to subschemas."] # [doc = ""] # [doc = " This is useful for dialects of JSON Schema (e.g. OpenAPI 3.0) that do not support booleans as"] # [doc = " schemas."] # [derive (Debug , Clone , Default)] # [non_exhaustive] pub struct ReplaceBoolSchemas { # [doc = " When set to `true`, a schema's `additionalProperties` property will not be changed from a"] # [doc = " boolean."] # [doc = ""] # [doc = " Defaults to `false`."] pub skip_additional_properties : bool , }
};
}
