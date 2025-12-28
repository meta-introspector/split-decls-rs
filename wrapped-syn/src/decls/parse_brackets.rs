macro_rules! deps {
    () => {
        ParseBuffer!();
        Brackets!();
        Result!();
    };
}

macro_rules! parse_brackets {
    () => {
        deps!();
        # [doc (hidden)] pub fn parse_brackets < 'a > (input : & ParseBuffer < 'a >) -> Result < Brackets < 'a > > { parse_delimited (input , Delimiter :: Bracket) . map (| (span , content) | Brackets { token : token :: Bracket (span) , content , }) }
    };
}

parse_brackets!()