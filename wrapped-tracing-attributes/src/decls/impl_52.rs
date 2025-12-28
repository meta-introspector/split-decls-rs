macro_rules! deps {
    () => {
        MaybeItemFn!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [doc = " This parses a `TokenStream` into a `MaybeItemFn`"] # [doc = " (just like `ItemFn`, but skips parsing the body)."] impl Parse for MaybeItemFn { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let outer_attrs = input . call (Attribute :: parse_outer) ? ; let vis : Visibility = input . parse () ? ; let sig : Signature = input . parse () ? ; let inner_attrs = input . call (Attribute :: parse_inner) ? ; let block ; let brace_token = syn :: braced ! (block in input) ; let block : TokenStream = block . call (| buffer | buffer . parse ()) ? ; Ok (Self { outer_attrs , inner_attrs , vis , sig , brace_token , block , }) } }
    };
}

impl_52!();