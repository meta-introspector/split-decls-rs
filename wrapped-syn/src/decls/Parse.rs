macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
    };
}

macro_rules! Parse {
    () => {
        deps!();
        # [doc = " Parsing interface implemented by all types that can be parsed in a default"] # [doc = " way from a token stream."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about implementing and using"] # [doc = " the `Parse` trait."] # [doc = ""] # [doc = " [module documentation]: self"] pub trait Parse : Sized { fn parse (input : ParseStream) -> Result < Self > ; }
    };
}

Parse!();