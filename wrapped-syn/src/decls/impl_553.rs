macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
        ParseQuote!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl ParseQuote for Vec < Stmt > { fn parse (input : ParseStream) -> Result < Self > { Block :: parse_within (input) } }
    };
}

impl_553!()