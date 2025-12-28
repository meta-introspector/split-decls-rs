macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! derive {
    () => {
        deps!();
        # [doc = " Defines a derive function named `$outer` which parses its input"] # [doc = " `TokenStream` as a `DeriveInput` and then invokes the `$inner` function."] # [doc = ""] # [doc = " Note that the separate `$outer` parameter is required - proc macro functions"] # [doc = " are currently required to live at the crate root, and so the caller must"] # [doc = " specify the name in order to avoid name collisions."] macro_rules ! derive { ($ trait : ident => $ outer : ident => $ inner : ident) => { # [proc_macro_derive ($ trait , attributes (zerocopy))] pub fn $ outer (ts : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (ts as DeriveInput) ; let zerocopy_crate = match extract_zerocopy_crate (& ast . attrs) { Ok (zerocopy_crate) => zerocopy_crate , Err (e) => return e . into_compile_error () . into () , } ; $ inner (& ast , Trait ::$ trait , & zerocopy_crate) . into_ts () . into () } } ; }
    };
}

derive!()