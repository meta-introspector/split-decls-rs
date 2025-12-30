// Generated macro for impl_129 (impl)
macro_rules! Depcrate_format_descriptionimpl_129 {
() => {
// Module: crate::format_description
// Provides: {"impl_129"}
// Dependencies: {}
impl Span { # [must_use = "this does not modify the original value"] const fn shrink_to_start (& self) -> Self { Self { start : self . start , end : self . start , } } # [must_use = "this does not modify the original value"] const fn shrink_to_end (& self) -> Self { Self { start : self . end , end : self . end , } } # [must_use = "this does not modify the original value"] const fn shrink_to_before (& self , pos : u32) -> Self { Self { start : self . start , end : Location { byte : self . start . byte + pos - 1 , proc_span : self . start . proc_span , } , } } # [must_use = "this does not modify the original value"] fn shrink_to_after (& self , pos : u32) -> Self { Self { start : Location { byte : self . start . byte + pos + 1 , proc_span : self . start . proc_span , } , end : self . end , } } fn error (self , message : & 'static str) -> Error { Error { message , _span : unused (self) , proc_span : self . start . proc_span , } } }
};
}
