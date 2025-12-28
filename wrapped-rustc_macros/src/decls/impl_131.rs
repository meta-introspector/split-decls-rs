macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Parse for Input { fn parse (input : ParseStream < '_ >) -> Result < Self > { input . parse :: < kw :: Keywords > () ? ; let content ; braced ! (content in input) ; let keywords = Punctuated :: parse_terminated (& content) ? ; input . parse :: < kw :: Symbols > () ? ; let content ; braced ! (content in input) ; let symbols = Punctuated :: parse_terminated (& content) ? ; Ok (Input { keywords , symbols }) } }
    };
}

impl_131!();