// Generated macro for extension (function)
macro_rules! Depcrateextension {
() => {
// Module: crate
// Provides: {"extension"}
// Dependencies: {}
# [doc = " Derive an extension trait for a given impl block. The trait name"] # [doc = " goes into the parenthesized args of the macro, for greppability."] # [doc = " For example:"] # [doc = " ```"] # [doc = " use rustc_macros::extension;"] # [doc = " #[extension(pub trait Foo)]"] # [doc = " impl i32 { fn hello() {} }"] # [doc = " ```"] # [doc = ""] # [doc = " expands to:"] # [doc = " ```"] # [doc = " pub trait Foo { fn hello(); }"] # [doc = " impl Foo for i32 { fn hello() {} }"] # [doc = " ```"] # [proc_macro_attribute] pub fn extension (attr : TokenStream , input : TokenStream) -> TokenStream { extension :: extension (attr , input) }
};
}
