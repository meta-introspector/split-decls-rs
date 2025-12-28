macro_rules! deps {
    () => {
        Group!();
        Result!();
        ParseStream!();
    };
}

macro_rules! parse_delimiter {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub (crate) fn parse_delimiter (input : ParseStream) -> Result < (MacroDelimiter , TokenStream) > { input . step (| cursor | { if let Some ((TokenTree :: Group (g) , rest)) = cursor . token_tree () { let span = g . delim_span () ; let delimiter = match g . delimiter () { Delimiter :: Parenthesis => MacroDelimiter :: Paren (Paren (span)) , Delimiter :: Brace => MacroDelimiter :: Brace (Brace (span)) , Delimiter :: Bracket => MacroDelimiter :: Bracket (Bracket (span)) , Delimiter :: None => { return Err (cursor . error ("expected delimiter")) ; } } ; Ok (((delimiter , g . stream ()) , rest)) } else { Err (cursor . error ("expected delimiter")) } }) }
    };
}

parse_delimiter!();