// Generated macro for wrap_serialize_variant_with (function)
macro_rules! Depcrate_serwrap_serialize_variant_with {
() => {
// Module: crate::ser
// Provides: {"wrap_serialize_variant_with"}
// Dependencies: {}
fn wrap_serialize_variant_with (params : & Parameters , serialize_with : & syn :: ExprPath , variant : & Variant ,) -> TokenStream { let field_tys : Vec < _ > = variant . fields . iter () . map (| field | field . ty) . collect () ; let field_exprs : Vec < _ > = variant . fields . iter () . map (| field | { let id = match & field . member { Member :: Named (ident) => ident . clone () , Member :: Unnamed (member) => { Ident :: new (& format ! ("__field{}" , member . index) , Span :: call_site ()) } } ; quote ! (# id) }) . collect () ; wrap_serialize_with (params , serialize_with , field_tys . as_slice () , field_exprs . as_slice () ,) }
};
}
