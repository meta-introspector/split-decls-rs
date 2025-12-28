macro_rules! deps {
    () => {
        Parse!();
        ParseQuote!();
        Result!();
        ParseStream!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < T : Parse > ParseQuote for T { fn parse (input : ParseStream) -> Result < Self > { < T as Parse > :: parse (input) } }
    };
}

impl_546!()