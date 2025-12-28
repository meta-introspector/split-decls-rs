macro_rules! deps {
    () => {
        Lint!();
        LintId!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl std :: hash :: Hash for LintId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let ptr = self . lint as * const Lint ; ptr . hash (state) ; } }
    };
}

impl_156!();