// Generated macro for impl_25 (impl)
macro_rules! Depcrate_fieldimpl_25 {
() => {
// Module: crate::field
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > NamedField < 'a > { # [doc = " Create a new `NamedField` instance with the given name."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::NamedField;"] # [doc = ""] # [doc = " let field = NamedField::new(\"hello\");"] # [doc = " assert_eq!(\"hello\", field.name());"] # [doc = " ```"] pub const fn new (name : & 'a str) -> NamedField < 'a > { NamedField (name) } # [doc = " Returns the field name"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::NamedField;"] # [doc = ""] # [doc = " let field = NamedField::new(\"hello\");"] # [doc = " assert_eq!(\"hello\", field.name());"] # [doc = " ```"] pub const fn name (& self) -> & str { self . 0 } }
};
}
