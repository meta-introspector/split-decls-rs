macro_rules! deps {
    () => {
        Group!();
        Result!();
        ParseStream!();
        Parse!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Group { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | { if let Some ((group , rest)) = cursor . any_group_token () { if group . delimiter () != Delimiter :: None { return Ok ((group , rest)) ; } } Err (cursor . error ("expected group token")) }) } }
    };
}

impl_522!();