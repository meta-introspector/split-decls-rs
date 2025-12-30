// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl ProcMacroExpander for Issue17479ProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let TokenTree :: Leaf (Leaf :: Literal (lit)) = & subtree . 0 [1] else { return Err (ProcMacroExpansionError :: Panic ("incorrect Input" . into ())) ; } ; let symbol = & lit . symbol ; let span = lit . span ; Ok (quote ! { span => # symbol () }) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
