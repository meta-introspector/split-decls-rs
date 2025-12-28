macro_rules! deps {
    () => {
        Token!();
        TokenKind!();
        DatetimeParseError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Token < '_ > { fn is (& self , kind : TokenKind) -> Result < () , DatetimeParseError > { if self . kind == kind { Ok (()) } else { Err (DatetimeParseError :: new ()) } } }
    };
}

impl_20!();