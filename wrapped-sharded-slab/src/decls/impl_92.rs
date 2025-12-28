macro_rules! deps {
    () => {
        Generation!();
        Config!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < C : cfg :: Config > Eq for Generation < C > { }
    };
}

impl_92!()