macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
    };
}

macro_rules! ParseQuote {
    () => {
        deps!();
        # [doc (hidden)] pub trait ParseQuote : Sized { fn parse (input : ParseStream) -> Result < Self > ; }
    };
}

ParseQuote!();