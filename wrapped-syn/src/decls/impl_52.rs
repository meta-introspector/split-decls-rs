macro_rules! deps {
    () => {
        Group!();
        Result!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Debug for Group { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("Group") } }
    };
}

impl_52!();