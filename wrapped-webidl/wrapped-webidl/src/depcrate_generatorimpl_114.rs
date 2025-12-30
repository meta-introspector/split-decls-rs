// Generated macro for impl_114 (impl)
macro_rules! Depcrate_generatorimpl_114 {
() => {
// Module: crate::generator
// Provides: {"impl_114"}
// Dependencies: {}
impl Const { fn generate (& self , options : & Options , parent_name : & Ident , parent_js_name : & str , deprecated : & Option < Option < String > > ,) -> TokenStream { let name = & self . name ; let ty = & self . ty ; let js_name = & self . js_name ; let value = self . value . generate () ; let unstable = self . unstable ; let unstable_attr = maybe_unstable_attr (unstable) ; let unstable_docs = maybe_unstable_docs (unstable) ; let doc_comment = comment (format ! ("The `{parent_js_name}.{js_name}` const.") , & get_features_doc (options , parent_name . to_string ()) ,) ; let deprecated = deprecated . as_ref () . map (| msg | match msg { Some (msg) => quote ! (# [deprecated (note = # msg)]) , None => quote ! (# [deprecated]) , }) ; quote ! { # unstable_attr # doc_comment # unstable_docs # deprecated pub const # name : # ty = # value as # ty ; } } }
};
}
