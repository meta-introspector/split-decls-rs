macro_rules! deps {
    () => {
        Addr!();
        Config!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for Addr < C > { }
    };
}

impl_143!();