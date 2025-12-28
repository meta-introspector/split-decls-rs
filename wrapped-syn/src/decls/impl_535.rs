macro_rules! deps {
    () => {
        Nothing!();
        Result!();
    };
}

macro_rules! impl_535 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Debug for Nothing { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("Nothing") } }
    };
}

impl_535!()