macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Copy for Group { }
    };
}

impl_50!()