// Generated macro for impl_67 (impl)
macro_rules! Depcrate_tagimpl_67 {
() => {
// Module: crate::tag
// Provides: {"impl_67"}
// Dependencies: {}
impl Tag { # [doc = " Build a new [`Tag`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tracing_forest::Tag;"] # [doc = ""] # [doc = " let tag = Tag::builder()"] # [doc = "     .prefix(\"security\")"] # [doc = "     .suffix(\"critical\")"] # [doc = "     .icon('🔐')"] # [doc = "     .build();"] # [doc = " ```"] pub fn builder () -> Builder < () , () > { Builder { prefix : None , suffix : () , icon : () , } } # [doc = " Returns the prefix, if there is one."] pub const fn prefix (& self) -> Option < & 'static str > { self . prefix } # [doc = " Returns the suffix."] pub const fn suffix (& self) -> & 'static str { self . suffix } # [doc = " Returns the icon."] pub const fn icon (& self) -> char { self . icon } }
};
}
