macro_rules! deps {
    () => {
        Issue18898ProcMacroExpander!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl ProcMacroExpander for Issue18898ProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , def_site : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let span = subtree . token_trees () . flat_tokens () . last () . ok_or_else (| | ProcMacroExpansionError :: Panic ("malformed input" . to_owned ())) ? . first_span () ; let overly_long_subtree = quote ! { span => { let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; let a = 5 ; } } ; Ok (quote ! { def_site => fn foo () { # overly_long_subtree } }) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_29!();