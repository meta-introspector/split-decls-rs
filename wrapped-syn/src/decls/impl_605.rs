macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T , P > Hash for Punctuated < T , P > where T : Hash , P : Hash , { fn hash < H : Hasher > (& self , state : & mut H) { let Punctuated { inner , last } = self ; inner . hash (state) ; last . hash (state) ; } }
    };
}

impl_605!();