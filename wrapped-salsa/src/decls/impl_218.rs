macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl HashEqLike < & Path > for PathBuf { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & & Path) -> bool { self == data } }
    };
}

impl_218!();