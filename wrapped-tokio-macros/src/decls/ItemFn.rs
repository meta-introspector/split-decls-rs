macro_rules! ItemFn {
    () => {
        struct ItemFn { outer_attrs : Vec < Attribute > , vis : Visibility , sig : Signature , brace_token : syn :: token :: Brace , inner_attrs : Vec < Attribute > , stmts : Vec < proc_macro2 :: TokenStream > , }
    };
}

ItemFn!();