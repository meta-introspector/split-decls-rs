macro_rules! deps {
    () => {
        IdentityProcMacroExpander!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl ProcMacroExpander for IdentityProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { Ok (subtree . clone ()) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_15!();