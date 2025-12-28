macro_rules! deps {
    () => {
        ParseStream!();
        ParseQuote!();
        Result!();
    };
}

macro_rules! impl_550 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl ParseQuote for Pat { fn parse (input : ParseStream) -> Result < Self > { Pat :: parse_multi_with_leading_vert (input) } }
    };
}

impl_550!()