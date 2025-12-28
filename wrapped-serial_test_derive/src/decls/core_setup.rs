macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! core_setup {
    () => {
        deps!();
        # [allow (clippy :: cmp_owned)] fn core_setup (input : proc_macro2 :: TokenStream , config : & Config , prefix : & str , kind : & str ,) -> proc_macro2 :: TokenStream { let fn_ast : SynResult < syn :: ItemFn > = syn :: parse2 (input . clone ()) ; if let Ok (ast) = fn_ast { return fn_setup (ast , config , prefix , kind) ; } ; let mod_ast : SynResult < syn :: ItemMod > = syn :: parse2 (input) ; match mod_ast { Ok (mut ast) => { let new_content = ast . content . clone () . map (| (brace , items) | { let new_items = items . into_iter () . map (| item | match item { syn :: Item :: Fn (item_fn) if item_fn . attrs . iter () . any (| attr | { attr . meta . path () . segments . iter () . map (| s | s . ident . to_string ()) . collect :: < Vec < String > > () . join ("::") . contains ("test") }) => { let tokens = fn_setup (item_fn , config , prefix , kind) ; let token_display = format ! ("tokens: {tokens}") ; syn :: parse2 (tokens) . expect (& token_display) } other => other , }) . collect () ; (brace , new_items) }) ; if let Some (nc) = new_content { ast . content . replace (nc) ; } ast . attrs . retain (| attr | { attr . meta . path () . segments . first () . unwrap () . ident . to_string () != "serial" }) ; ast . into_token_stream () } Err (_) => { panic ! ("Attribute applied to something other than mod or fn!") ; } } }
    };
}

core_setup!()