macro_rules! deps {
    () => {
        LintId!();
        Lint!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl std :: hash :: Hash for LintId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let ptr = self . lint as * const Lint ; ptr . hash (state) ; } }
    };
}

impl_20!()