macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
    };
}

macro_rules! ParseQuote {
    () => {
        deps!();
        # [doc (hidden)] pub trait ParseQuote : Sized { fn parse (input : ParseStream) -> Result < Self > ; }
    };
}

ParseQuote!()