macro_rules! deps {
    () => {
        RootDirFilter!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < F : Fn (& Path) -> bool > RootDirFilter for F { }
    };
}

impl_125!();