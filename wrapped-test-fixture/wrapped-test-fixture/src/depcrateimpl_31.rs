// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl ProcMacroExpander for AttributeInputReplaceProcMacroExpander { fn expand (& self , _ : & TopSubtree , attrs : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { attrs . cloned () . ok_or_else (| | ProcMacroExpansionError :: Panic ("Expected attribute input" . into ())) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
