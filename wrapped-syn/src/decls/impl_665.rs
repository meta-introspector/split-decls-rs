macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Copy for Pair < T , P > where T : Copy , P : Copy , { }
    };
}

impl_665!();