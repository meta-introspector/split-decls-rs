macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < T : Parse > Parse for List < T > { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut list = Vec :: new () ; while ! input . is_empty () { list . push (input . parse () ?) ; } Ok (List (list)) } }
    };
}

impl_103!()