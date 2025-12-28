macro_rules! deps {
    () => {
        FutureExt!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T : Future + ? Sized > FutureExt for T { }
    };
}

impl_120!();