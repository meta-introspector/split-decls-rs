// Generated macro for impl_10 (impl)
macro_rules! Depcrate_attrimpl_10 {
() => {
// Module: crate::attr
// Provides: {"impl_10"}
// Dependencies: {}
impl Attrs { pub (crate) fn rename (& self , original : & Ident) -> syn :: LitStr { self . rename . as_ref () . map_or_else (| | syn :: LitStr :: new (& original . to_string () , original . span ()) , | (_ , l) | l . clone () ,) } pub (crate) fn transparent (& self) -> bool { self . transparent . is_some () } pub (crate) fn skip (& self) -> bool { self . skip . is_some () } }
};
}
