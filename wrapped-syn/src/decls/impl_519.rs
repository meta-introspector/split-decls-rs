macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        Parse!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl < T : Parse + Token > Parse for Option < T > { fn parse (input : ParseStream) -> Result < Self > { if T :: peek (input . cursor ()) { Ok (Some (input . parse () ?)) } else { Ok (None) } } }
    };
}

impl_519!();