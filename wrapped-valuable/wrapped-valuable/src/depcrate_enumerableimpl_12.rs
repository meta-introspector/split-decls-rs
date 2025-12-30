// Generated macro for impl_12 (impl)
macro_rules! Depcrate_enumerableimpl_12 {
() => {
// Module: crate::enumerable
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > VariantDef < 'a > { # [doc = " Creates a new `VariantDef` instance."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::{Fields, VariantDef};"] # [doc = ""] # [doc = " let def = VariantDef::new(\"Foo\", Fields::Unnamed(2));"] # [doc = " ```"] pub const fn new (name : & 'a str , fields : Fields < 'a >) -> VariantDef < 'a > { VariantDef { name , fields } } # [doc = " Returns the variant's name"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::{Fields, VariantDef};"] # [doc = ""] # [doc = " let def = VariantDef::new(\"Foo\", Fields::Unnamed(2));"] # [doc = " assert_eq!(\"Foo\", def.name());"] # [doc = " ```"] pub fn name (& self) -> & str { self . name } # [doc = " Returns the variant's fields"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::{Fields, VariantDef};"] # [doc = ""] # [doc = " let def = VariantDef::new(\"Foo\", Fields::Unnamed(3));"] # [doc = " assert!(matches!(def.fields(), Fields::Unnamed(_)));"] # [doc = " ```"] pub fn fields (& self) -> & Fields < '_ > { & self . fields } }
};
}
