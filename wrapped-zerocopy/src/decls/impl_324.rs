macro_rules! deps {
    () => {
        Exclusive!();
        Aliasing!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl Aliasing for Exclusive { const IS_EXCLUSIVE : bool = true ; }
    };
}

impl_324!()