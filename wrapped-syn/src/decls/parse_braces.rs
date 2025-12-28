macro_rules! deps {
    () => {
        ParseBuffer!();
        Result!();
        Braces!();
    };
}

macro_rules! parse_braces {
    () => {
        deps!();
        # [doc (hidden)] pub fn parse_braces < 'a > (input : & ParseBuffer < 'a >) -> Result < Braces < 'a > > { parse_delimited (input , Delimiter :: Brace) . map (| (span , content) | Braces { token : token :: Brace (span) , content , }) }
    };
}

parse_braces!();