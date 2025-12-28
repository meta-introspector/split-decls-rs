macro_rules! deps {
    () => {
        Result!();
        Parse!();
        Nothing!();
        ParseStream!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl Parse for Nothing { fn parse (_input : ParseStream) -> Result < Self > { Ok (Nothing) } }
    };
}

impl_531!();