// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl ProcMacroExpander for ShortenProcMacroExpander { fn expand (& self , input : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let mut result = input . 0 . clone () ; for it in & mut result { if let TokenTree :: Leaf (leaf) = it { modify_leaf (leaf) } } return Ok (tt :: TopSubtree (result)) ; fn modify_leaf (leaf : & mut Leaf) { match leaf { Leaf :: Literal (it) => { it . symbol = Symbol :: empty () ; } Leaf :: Punct (_) => { } Leaf :: Ident (it) => { it . sym = Symbol :: intern (& it . sym . as_str () . chars () . take (1) . collect :: < String > ()) ; } } } } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
