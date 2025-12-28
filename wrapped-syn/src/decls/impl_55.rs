macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for Group { fn hash < H : Hasher > (& self , _state : & mut H) { } }
    };
}

impl_55!();