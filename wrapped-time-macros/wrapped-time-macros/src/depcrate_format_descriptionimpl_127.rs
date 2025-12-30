// Generated macro for impl_127 (impl)
macro_rules! Depcrate_format_descriptionimpl_127 {
() => {
// Module: crate::format_description
// Provides: {"impl_127"}
// Dependencies: {}
impl Location { fn to (self , end : Self) -> Span { Span { start : self , end } } # [must_use = "this does not modify the original value"] fn offset (& self , offset : u32) -> Self { Self { byte : self . byte + offset , proc_span : self . proc_span , } } fn error (self , message : & 'static str) -> Error { Error { message , _span : unused (Span { start : self , end : self , }) , proc_span : self . proc_span , } } }
};
}
