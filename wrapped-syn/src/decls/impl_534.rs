macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Copy for Nothing { }
    };
}

impl_534!()