macro_rules! deps {
    () => {
        Parse!();
        ParseStream!();
        Result!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Punct { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . punct () { Some ((punct , rest)) => Ok ((punct , rest)) , None => Err (cursor . error ("expected punctuation token")) , }) } }
    };
}

impl_523!()