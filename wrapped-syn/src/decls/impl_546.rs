macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        ParseQuote!();
        Parse!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < T : Parse > ParseQuote for T { fn parse (input : ParseStream) -> Result < Self > { < T as Parse > :: parse (input) } }
    };
}

impl_546!();