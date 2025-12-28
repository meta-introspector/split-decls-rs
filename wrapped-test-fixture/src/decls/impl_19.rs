macro_rules! deps {
    () => {
        AttributeInputReplaceProcMacroExpander!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ProcMacroExpander for AttributeInputReplaceProcMacroExpander { fn expand (& self , _ : & TopSubtree , attrs : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { attrs . cloned () . ok_or_else (| | ProcMacroExpansionError :: Panic ("Expected attribute input" . into ())) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_19!();