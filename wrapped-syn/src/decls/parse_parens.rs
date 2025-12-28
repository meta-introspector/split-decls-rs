macro_rules! deps {
    () => {
        Parens!();
        ParseBuffer!();
        Result!();
    };
}

macro_rules! parse_parens {
    () => {
        deps!();
        # [doc (hidden)] pub fn parse_parens < 'a > (input : & ParseBuffer < 'a >) -> Result < Parens < 'a > > { parse_delimited (input , Delimiter :: Parenthesis) . map (| (span , content) | Parens { token : token :: Paren (span) , content , }) }
    };
}

parse_parens!();