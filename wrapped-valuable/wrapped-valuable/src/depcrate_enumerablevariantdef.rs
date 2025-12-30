// Generated macro for VariantDef (struct)
macro_rules! Depcrate_enumerableVariantDef {
() => {
// Module: crate::enumerable
// Provides: {"VariantDef"}
// Dependencies: {}
# [doc = " An enum variant definition."] # [doc = ""] # [doc = " Included with [`EnumDef`] returned by [`Enumerable::definition()`],"] # [doc = " `VariantDef` provides the caller with information about a specific variant."] # [derive (Debug)] pub struct VariantDef < 'a > { # [doc = " Variant name"] name : & 'a str , # [doc = " Variant fields"] fields : Fields < 'a > , }
};
}
