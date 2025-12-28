macro_rules! deps {
    () => {
        Fields!();
        Field!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Parse for Fields { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < kw :: fields > () ; let content ; let _ = syn :: parenthesized ! (content in input) ; let fields = content . parse_terminated (Field :: parse , Token ! [,]) ? ; Ok (Self (fields)) } }
    };
}

impl_20!();