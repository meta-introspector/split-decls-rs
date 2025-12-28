macro_rules! deps {
    () => {
        Result!();
        ParseQuote!();
        ParseStream!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        # [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Attribute { fn parse (input : ParseStream) -> Result < Self > { if input . peek (Token ! [#]) && input . peek2 (Token ! [!]) { attr :: parsing :: single_parse_inner (input) } else { attr :: parsing :: single_parse_outer (input) } } }
    };
}

impl_547!();