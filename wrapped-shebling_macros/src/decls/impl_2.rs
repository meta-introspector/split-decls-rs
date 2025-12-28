macro_rules! deps {
    () => {
        SheblingArgs!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Parse for SheblingArgs { fn parse (input : ParseStream) -> SynResult < Self > { let path_key : Ident = input . parse () ? ; if path_key != "path" { return Err (input . error ("Expected 'path' as the argument")) ; } let _colon1 = input . parse () ? ; let path = input . parse () ? ; Ok (SheblingArgs { path_key , _colon1 , path }) } }
    };
}

impl_2!();