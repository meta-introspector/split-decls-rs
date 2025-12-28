macro_rules! deps {
    () => {
        ParseQuote!();
        ParseStream!();
        Result!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl ParseQuote for Vec < Arm > { fn parse (input : ParseStream) -> Result < Self > { Arm :: parse_multiple (input) } }
    };
}

impl_554!()