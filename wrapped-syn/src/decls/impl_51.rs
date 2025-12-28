macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for Group { fn clone (& self) -> Self { * self } }
    };
}

impl_51!()