macro_rules! deps {
    () => {
        TomlWrite!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < W > TomlWrite for W where W : core :: fmt :: Write { }
    };
}

impl_75!()