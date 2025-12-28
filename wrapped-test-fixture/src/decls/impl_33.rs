macro_rules! deps {
    () => {
        GenerateSuffixedTypeProcMacroExpander!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl ProcMacroExpander for GenerateSuffixedTypeProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _attrs : Option < & TopSubtree > , _env : & Env , _def_site : Span , call_site : Span , _mixed_site : Span , _current_dir : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let TokenTree :: Leaf (Leaf :: Ident (ident)) = & subtree . 0 [1] else { return Err (ProcMacroExpansionError :: Panic ("incorrect Input" . into ())) ; } ; let ident = match ident . sym . as_str () { "struct" => { let TokenTree :: Leaf (Leaf :: Ident (ident)) = & subtree . 0 [2] else { return Err (ProcMacroExpansionError :: Panic ("incorrect Input" . into ())) ; } ; ident } "enum" => { let TokenTree :: Leaf (Leaf :: Ident (ident)) = & subtree . 0 [4] else { return Err (ProcMacroExpansionError :: Panic ("incorrect Input" . into ())) ; } ; ident } _ => { return Err (ProcMacroExpansionError :: Panic ("incorrect Input" . into ())) ; } } ; let generated_ident = tt :: Ident { sym : Symbol :: intern (& format ! ("{}Suffix" , ident . sym)) , span : ident . span , is_raw : tt :: IdentIsRaw :: No , } ; let ret = quote ! { call_site => # subtree struct # generated_ident ; } ; Ok (ret) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_33!();