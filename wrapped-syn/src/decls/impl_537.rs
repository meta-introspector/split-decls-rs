macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for Nothing { fn eq (& self , _other : & Self) -> bool { true } }
    };
}

impl_537!();