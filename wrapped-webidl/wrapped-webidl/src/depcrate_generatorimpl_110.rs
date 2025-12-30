// Generated macro for impl_110 (impl)
macro_rules! Depcrate_generatorimpl_110 {
() => {
// Module: crate::generator
// Provides: {"impl_110"}
// Dependencies: {}
impl Enum { pub fn generate (& self , options : & Options) -> TokenStream { let Enum { name , variants , unstable , } = self ; let unstable_attr = maybe_unstable_attr (* unstable) ; let unstable_docs = maybe_unstable_docs (* unstable) ; let doc_comment = comment (format ! ("The `{name}` enum.") , & get_features_doc (options , name . to_string ()) ,) ; let variants = variants . iter () . map (| variant | variant . generate ()) . collect :: < Vec < _ > > () ; quote ! { #! [allow (unused_imports)] #! [allow (clippy :: all)] use wasm_bindgen :: prelude ::*; # unstable_attr # [wasm_bindgen] # doc_comment # unstable_docs # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum # name { # (# variants) ,* } } } }
};
}
