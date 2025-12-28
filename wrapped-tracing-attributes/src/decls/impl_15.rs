macro_rules! deps {
    () => {
        Skips!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Parse for Skips { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < kw :: skip > () ; let content ; let _ = syn :: parenthesized ! (content in input) ; let names = content . parse_terminated (Ident :: parse_any , Token ! [,]) ? ; let mut skips = HashSet :: new () ; for name in names { if skips . contains (& name) { return Err (syn :: Error :: new (name . span () , "tried to skip the same field twice" ,)) ; } else { skips . insert (name) ; } } Ok (Self (skips)) } }
    };
}

impl_15!()