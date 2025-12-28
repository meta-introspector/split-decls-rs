macro_rules! deps {
    () => {
        Result!();
        ParseQuote!();
        ParseStream!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        # [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Vec < Attribute > { fn parse (input : ParseStream) -> Result < Self > { let mut attrs = Vec :: new () ; while ! input . is_empty () { attrs . push (ParseQuote :: parse (input) ?) ; } Ok (attrs) } }
    };
}

impl_548!()