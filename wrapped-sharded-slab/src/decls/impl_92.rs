macro_rules! deps {
    () => {
        Config!();
        Generation!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < C : cfg :: Config > Eq for Generation < C > { }
    };
}

impl_92!();