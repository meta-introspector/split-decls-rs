macro_rules! deps {
    () => {
        ParseBuffer!();
        Result!();
        Group!();
    };
}

macro_rules! parse_group {
    () => {
        deps!();
        # [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn parse_group < 'a > (input : & ParseBuffer < 'a >) -> Result < Group < 'a > > { parse_delimited (input , Delimiter :: None) . map (| (span , content) | Group { token : token :: Group (span . join ()) , content , }) }
    };
}

parse_group!()