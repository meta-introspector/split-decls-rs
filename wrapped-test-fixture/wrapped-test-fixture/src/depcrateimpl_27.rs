// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl ProcMacroExpander for IdentityProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { Ok (subtree . clone ()) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
