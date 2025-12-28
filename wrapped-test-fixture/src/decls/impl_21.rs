macro_rules! deps {
    () => {
        Issue18840ProcMacroExpander!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl ProcMacroExpander for Issue18840ProcMacroExpander { fn expand (& self , fn_ : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , def_site : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let fixed_up_span = fn_ . token_trees () . flat_tokens () [5] . first_span () ; let mut result = quote ! { fixed_up_span => :: core :: compile_error ! { "my cool compile_error!" } } ; let top_subtree_delimiter_mut = result . top_subtree_delimiter_mut () ; top_subtree_delimiter_mut . open = def_site ; top_subtree_delimiter_mut . close = def_site ; Ok (result) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_21!();