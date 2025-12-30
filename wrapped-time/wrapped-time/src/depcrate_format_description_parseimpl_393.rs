// Generated macro for impl_393 (impl)
macro_rules! Depcrate_format_description_parseimpl_393 {
() => {
// Module: crate::format_description::parse
// Provides: {"impl_393"}
// Dependencies: {}
impl Span { # [doc = " Obtain a `Span` pointing at the start of the pre-existing span."] # [must_use = "this does not modify the original value"] # [inline] const fn shrink_to_start (& self) -> Self { Self { start : self . start , end : self . start , } } # [doc = " Obtain a `Span` pointing at the end of the pre-existing span."] # [must_use = "this does not modify the original value"] const fn shrink_to_end (& self) -> Self { Self { start : self . end , end : self . end , } } # [doc = " Obtain a `Span` that ends before the provided position of the pre-existing span."] # [must_use = "this does not modify the original value"] # [inline] const fn shrink_to_before (& self , pos : u32) -> Self { Self { start : self . start , end : Location { byte : self . start . byte + pos - 1 , } , } } # [doc = " Obtain a `Span` that starts after provided position to the end of the pre-existing span."] # [must_use = "this does not modify the original value"] # [inline] const fn shrink_to_after (& self , pos : u32) -> Self { Self { start : Location { byte : self . start . byte + pos + 1 , } , end : self . end , } } # [doc = " Create an error with the provided message at this span."] # [inline] const fn error (self , message : & 'static str) -> ErrorInner { ErrorInner { _message : message , _span : self , } } }
};
}
