// Generated macro for impl_391 (impl)
macro_rules! Depcrate_format_description_parseimpl_391 {
() => {
// Module: crate::format_description::parse
// Provides: {"impl_391"}
// Dependencies: {}
impl Location { # [doc = " Create a new [`Span`] from `self` to `other`."] # [inline] const fn to (self , end : Self) -> Span { Span { start : self , end } } # [doc = " Create a new [`Span`] consisting entirely of `self`."] # [inline] const fn to_self (self) -> Span { Span { start : self , end : self , } } # [doc = " Offset the location by the provided amount."] # [doc = ""] # [doc = " Note that this assumes the resulting location is on the same line as the original location."] # [must_use = "this does not modify the original value"] # [inline] const fn offset (& self , offset : u32) -> Self { Self { byte : self . byte + offset , } } # [doc = " Create an error with the provided message at this location."] # [inline] const fn error (self , message : & 'static str) -> ErrorInner { ErrorInner { _message : message , _span : Span { start : self , end : self , } , } } }
};
}
