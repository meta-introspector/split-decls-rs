macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_603 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T , P > Eq for Punctuated < T , P > where T : Eq , P : Eq , { }
    };
}

impl_603!();