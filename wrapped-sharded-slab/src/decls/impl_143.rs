macro_rules! deps {
    () => {
        Config!();
        Addr!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for Addr < C > { }
    };
}

impl_143!()