macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl std :: panic :: UnwindSafe for CancellationToken { }
    };
}

impl_34!();