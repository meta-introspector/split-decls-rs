macro_rules! deps {
    () => {
        StrArg!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T : Parse > Parse for StrArg < T > { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < T > () ? ; let _ = input . parse :: < Token ! [=] > () ? ; let value = input . parse () ? ; Ok (Self { value , _p : std :: marker :: PhantomData , }) } }
    };
}

impl_11!();