// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl ProcMacroExpander for DisallowCfgProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { for tt in subtree . token_trees () . flat_tokens () { if let tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (ident)) = tt && (ident . sym == sym :: cfg || ident . sym == sym :: cfg_attr) { return Err (ProcMacroExpansionError :: Panic ("cfg or cfg_attr found in DisallowCfgProcMacroExpander" . to_owned () ,)) ; } } Ok (subtree . clone ()) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
