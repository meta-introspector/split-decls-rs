macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl std :: panic :: RefUnwindSafe for CancellationToken { }
    };
}

impl_35!()