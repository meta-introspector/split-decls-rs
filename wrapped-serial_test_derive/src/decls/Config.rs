macro_rules! deps {
    () => {
        QuoteOption!();
    };
}

macro_rules! Config {
    () => {
        deps!();
        # [derive (Default , Debug)] struct Config { names : Vec < String > , path : QuoteOption < String > , crate_ident : Vec < TokenTree > , }
    };
}

Config!();