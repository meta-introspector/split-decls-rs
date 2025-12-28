macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for Group { fn eq (& self , _other : & Group) -> bool { true } }
    };
}

impl_54!()