// Generated macro for pretend_fields_used_struct_packed (function)
macro_rules! Depcrate_pretendpretend_fields_used_struct_packed {
() => {
// Module: crate::pretend
// Provides: {"pretend_fields_used_struct_packed"}
// Dependencies: {}
fn pretend_fields_used_struct_packed (cont : & Container , fields : & [Field]) -> TokenStream { let type_ident = & cont . ident ; let (_ , ty_generics , _) = cont . generics . split_for_impl () ; let members = fields . iter () . map (| field | & field . member) . collect :: < Vec < _ > > () ; let private2 = private ; quote ! { match _serde ::# private :: None ::<&# type_ident # ty_generics > { _serde ::# private :: Some (__v @ # type_ident { # (# members : _) ,* }) => { # (let _ = _serde ::# private2 :: ptr :: addr_of ! (__v .# members) ;) * } _ => { } } } }
};
}
