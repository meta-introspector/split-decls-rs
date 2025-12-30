// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl ProcMacroExpander for Issue18089ProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , call_site : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let tt :: TokenTree :: Leaf (macro_name) = & subtree . 0 [2] else { return Err (ProcMacroExpansionError :: Panic ("incorrect input" . to_owned ())) ; } ; Ok (quote ! { call_site => # [macro_export] macro_rules ! my_macro___ { ($ ($ token : tt) *) => { { } } ; } pub use my_macro___ as # macro_name ; # subtree }) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
