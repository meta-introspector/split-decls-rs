// Generated macro for RemoveRefSiblings (struct)
macro_rules! Depcrate_transformRemoveRefSiblings {
() => {
// Module: crate::transform
// Provides: {"RemoveRefSiblings"}
// Dependencies: {}
# [doc = " Restructures JSON Schema objects so that the `$ref` property will never appear alongside any"] # [doc = " other properties."] # [doc = ""] # [doc = " This also applies to subschemas."] # [doc = ""] # [doc = " This is useful for versions of JSON Schema (e.g. Draft 7) that do not support other properties"] # [doc = " alongside `$ref`."] # [derive (Debug , Clone , Default)] # [non_exhaustive] pub struct RemoveRefSiblings ;
};
}
