macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for Nothing { fn clone (& self) -> Self { * self } }
    };
}

impl_533!()