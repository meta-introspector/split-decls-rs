// Generated macro for Variant (enum)
macro_rules! Depcrate_enumerableVariant {
() => {
// Module: crate::enumerable
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " An enum variant"] # [doc = ""] # [doc = " Returned by [`Enumerable::variant()`], `Variant` represents a single enum"] # [doc = " variant."] # [derive (Debug)] pub enum Variant < 'a > { # [doc = " The variant is statically defined by the associated enum."] Static (& 'static VariantDef < 'static >) , # [doc = " The variant is dynamically defined and not included as part of"] # [doc = " [`Enumerable::definition()`]."] Dynamic (VariantDef < 'a >) , }
};
}
