macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < C : cfg :: Config > Eq for RefCount < C > { }
    };
}

impl_107!();