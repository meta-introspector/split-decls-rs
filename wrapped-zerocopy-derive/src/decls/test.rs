macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [doc = " Test that the given derive input expands to the expected output."] # [doc = ""] # [doc = " Equality is tested by formatting both token streams using `prettyplease` and"] # [doc = " performing string equality on the results. This has the effect of making the"] # [doc = " tests less brittle and robust against meaningless formatting changes."] macro_rules ! test { ($ name : ident { $ ($ i : tt) * } expands to { $ ($ o : tt) * }) => { { # [allow (dead_code)] fn ensure_compiles () { $ ($ i) * $ ($ o) * } test ! ($ name { $ ($ i) * } expands to { $ ($ o) * } no_build) ; } } ; ($ name : ident { $ ($ i : tt) * } expands to { $ ($ o : tt) * } no_build) => { { let ts : proc_macro2 :: TokenStream = quote :: quote ! ($ ($ i) *) ; let ast = syn :: parse2 ::< syn :: DeriveInput > (ts) . unwrap () ; let res = $ name (& ast , crate :: Trait ::$ name , & syn :: parse_quote ! (:: zerocopy)) ; let expected_toks = quote :: quote ! ($ ($ o) *) ; assert_eq_streams (expected_toks . into () , res . into_ts () . into ()) ; } } ; }
    };
}

test!()