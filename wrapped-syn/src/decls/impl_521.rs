macro_rules! deps {
    () => {
        Result!();
        Parse!();
        ParseStream!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for TokenTree { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . token_tree () { Some ((tt , rest)) => Ok ((tt , rest)) , None => Err (cursor . error ("expected token tree")) , }) } }
    };
}

impl_521!()