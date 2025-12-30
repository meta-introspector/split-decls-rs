// Generated macro for pretend_fields_used_struct (function)
macro_rules! Depcrate_pretendpretend_fields_used_struct {
() => {
// Module: crate::pretend
// Provides: {"pretend_fields_used_struct"}
// Dependencies: {}
fn pretend_fields_used_struct (cont : & Container , fields : & [Field]) -> TokenStream { let type_ident = & cont . ident ; let (_ , ty_generics , _) = cont . generics . split_for_impl () ; let members = fields . iter () . map (| field | & field . member) ; let placeholders = (0usize ..) . map (| i | format_ident ! ("__v{}" , i)) ; quote ! { match _serde ::# private :: None ::<&# type_ident # ty_generics > { _serde ::# private :: Some (# type_ident { # (# members : # placeholders) ,* }) => { } _ => { } } } }
};
}
