// Generated macro for ReplacePrefixItems (struct)
macro_rules! Depcrate_transformReplacePrefixItems {
() => {
// Module: crate::transform
// Provides: {"ReplacePrefixItems"}
// Dependencies: {}
# [doc = " Rename the `prefixItems` schema property to `items`."] # [doc = ""] # [doc = " This also applies to subschemas."] # [doc = ""] # [doc = " If the schema contains both `prefixItems` and `items`, then this additionally renames `items` to"] # [doc = " `additionalItems`."] # [doc = ""] # [doc = " This is useful for versions of JSON Schema (e.g. Draft 7) that do not support the `prefixItems`"] # [doc = " property."] # [derive (Debug , Clone , Default)] # [non_exhaustive] pub struct ReplacePrefixItems ;
};
}
