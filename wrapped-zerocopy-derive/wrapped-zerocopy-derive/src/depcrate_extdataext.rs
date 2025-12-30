// Generated macro for DataExt (trait)
macro_rules! Depcrate_extDataExt {
() => {
// Module: crate::ext
// Provides: {"DataExt"}
// Dependencies: {}
pub (crate) trait DataExt { # [doc = " Extracts the names and types of all fields. For enums, extracts the"] # [doc = " names and types of fields from each variant. For tuple structs, the"] # [doc = " names are the indices used to index into the struct (ie, `0`, `1`, etc)."] # [doc = ""] # [doc = " FIXME: Extracting field names for enums doesn't really make sense. Types"] # [doc = " makes sense because we don't care about where they live - we just care"] # [doc = " about transitive ownership. But for field names, we'd only use them when"] # [doc = " generating is_bit_valid, which cares about where they live."] fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > ; fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > ; fn tag (& self) -> Option < Ident > ; }
};
}
