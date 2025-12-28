macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for Nothing { fn hash < H : Hasher > (& self , _state : & mut H) { } }
    };
}

impl_538!()