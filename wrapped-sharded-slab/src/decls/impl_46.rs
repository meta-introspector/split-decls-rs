macro_rules! deps {
    () => {
        Config!();
        CfgPrivate!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < C : Config > CfgPrivate for C { }
    };
}

impl_46!()