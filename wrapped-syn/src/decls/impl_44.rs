macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
        Parse!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Underscore { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | { if let Some ((ident , rest)) = cursor . ident () { if ident == "_" { return Ok ((Underscore (ident . span ()) , rest)) ; } } if let Some ((punct , rest)) = cursor . punct () { if punct . as_char () == '_' { return Ok ((Underscore (punct . span ()) , rest)) ; } } Err (cursor . error ("expected `_`")) }) } }
    };
}

impl_44!();