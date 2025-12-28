macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl cmp :: Eq for Group { }
    };
}

impl_53!()