macro_rules! deps {
    () => {
        ParseQuote!();
        ParseStream!();
        Result!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl ParseQuote for Box < Pat > { fn parse (input : ParseStream) -> Result < Self > { < Pat as ParseQuote > :: parse (input) . map (Box :: new) } }
    };
}

impl_551!()