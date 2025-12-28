macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        Parse!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Literal { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . literal () { Some ((literal , rest)) => Ok ((literal , rest)) , None => Err (cursor . error ("expected literal token")) , }) } }
    };
}

impl_524!()