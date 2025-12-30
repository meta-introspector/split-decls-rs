// Generated macro for pretend_fields_used_enum (function)
macro_rules! Depcrate_pretendpretend_fields_used_enum {
() => {
// Module: crate::pretend
// Provides: {"pretend_fields_used_enum"}
// Dependencies: {}
fn pretend_fields_used_enum (cont : & Container , variants : & [Variant]) -> TokenStream { let type_ident = & cont . ident ; let (_ , ty_generics , _) = cont . generics . split_for_impl () ; let patterns = variants . iter () . filter_map (| variant | match variant . style { Style :: Struct | Style :: Tuple | Style :: Newtype => { let variant_ident = & variant . ident ; let members = variant . fields . iter () . map (| field | & field . member) ; let placeholders = (0usize ..) . map (| i | format_ident ! ("__v{}" , i)) ; Some (quote ! (# type_ident ::# variant_ident { # (# members : # placeholders) ,* })) } Style :: Unit => None , }) . collect :: < Vec < _ > > () ; quote ! { match _serde :: __private :: None ::<&# type_ident # ty_generics > { # (_serde :: __private :: Some (# patterns) => { }) * _ => { } } } }
};
}
