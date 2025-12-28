macro_rules! deps {
    () => {
        Pretty!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl Pretty { pub (crate) fn new () -> Self { Self { in_value : false } } }
    };
}

impl_378!();