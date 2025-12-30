// Generated macro for declare_component (macro)
macro_rules! Depcrate_format_description_public_componentdeclare_component {
() => {
// Module: crate::format_description::public::component
// Provides: {"declare_component"}
// Dependencies: {}
macro_rules ! declare_component { ($ ($ name : ident) *) => { pub (crate) enum Component { $ ($ name (modifier ::$ name) ,) * } impl ToTokenStream for Component { fn append_to (self , ts : & mut TokenStream) { let mut mts = TokenStream :: new () ; let component = match self { $ (Self ::$ name (modifier) => { modifier . append_to (& mut mts) ; stringify ! ($ name) }) * } ; let component = Ident :: new (component , Span :: mixed_site ()) ; quote_append ! { ts Component ::# (component) (# S (mts)) } } } } ; }
};
}
