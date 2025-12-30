// Generated macro for impl_130 (impl)
macro_rules! Depcrate_generatorimpl_130 {
() => {
// Module: crate::generator
// Provides: {"impl_130"}
// Dependencies: {}
impl Namespace < '_ > { pub fn generate (& self , options : & Options) -> TokenStream { let Namespace { name , js_name , consts , functions , unstable , } = self ; let unstable_attr = maybe_unstable_attr (* unstable) ; let unstable_docs = maybe_unstable_docs (* unstable) ; let functions = functions . iter () . map (| x | x . generate (options , name , js_name . to_string ())) . collect :: < Vec < _ > > () ; let functions = if functions . is_empty () { None } else { Some (quote ! { # [wasm_bindgen] extern "C" { # (# functions) * } }) } ; let consts = consts . iter () . map (| x | x . generate (options , name , js_name , & None)) . collect :: < Vec < _ > > () ; quote ! { # unstable_attr # unstable_docs pub mod # name { #! [allow (unused_imports)] #! [allow (clippy :: all)] use super :: super ::*; use wasm_bindgen :: prelude ::*; # (# consts) * # functions } } } }
};
}
