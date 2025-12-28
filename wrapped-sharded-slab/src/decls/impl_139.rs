macro_rules! deps {
    () => {
        Addr!();
        Config!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < C : cfg :: Config > Eq for Addr < C > { }
    };
}

impl_139!();