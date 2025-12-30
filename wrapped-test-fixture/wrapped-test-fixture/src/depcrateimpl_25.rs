// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl ProcMacroExpander for IdentityProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { Ok (subtree . clone ()) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
