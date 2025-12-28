macro_rules! impl_113 {
    () => {
        impl < L , R > HasAttrs for Either < L , R > where L : HasAttrs , R : HasAttrs , { }
    };
}

impl_113!()