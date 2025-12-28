macro_rules! deps {
    () => {
        RealFileName!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Hash for RealFileName { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . remapped_path_if_available () . hash (state) } }
    };
}

impl_25!()