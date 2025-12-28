macro_rules! deps {
    () => {
        Keyword!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl Parse for Keyword { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; input . parse :: < Token ! [:] > () ? ; let value = input . parse () ? ; Ok (Keyword { name , value }) } }
    };
}

impl_125!()