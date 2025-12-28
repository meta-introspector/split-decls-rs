macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Guard { fn acquire () -> Self { Guard :: Locked (LOCK . lock () . unwrap_or_else (PoisonError :: into_inner)) } }
    };
}

impl_97!();