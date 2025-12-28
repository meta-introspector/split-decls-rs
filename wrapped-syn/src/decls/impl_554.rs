macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
        ParseQuote!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl ParseQuote for Vec < Arm > { fn parse (input : ParseStream) -> Result < Self > { Arm :: parse_multiple (input) } }
    };
}

impl_554!();