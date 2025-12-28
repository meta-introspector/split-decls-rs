macro_rules! deps {
    () => {
        Parse!();
        Result!();
        ParseQuote!();
        Punctuated!();
        ParseStream!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < T : Parse , P : Parse > ParseQuote for Punctuated < T , P > { fn parse (input : ParseStream) -> Result < Self > { Self :: parse_terminated (input) } }
    };
}

impl_552!()