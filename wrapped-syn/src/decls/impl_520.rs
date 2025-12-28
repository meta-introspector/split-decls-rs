macro_rules! deps {
    () => {
        Parse!();
        Cursor!();
        Result!();
        ParseStream!();
    };
}

macro_rules! impl_520 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for TokenStream { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | Ok ((cursor . token_stream () , Cursor :: empty ()))) } }
    };
}

impl_520!()