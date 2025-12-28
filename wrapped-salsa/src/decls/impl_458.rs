macro_rules! deps {
    () => {
        ZalsaLocal!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl std :: panic :: RefUnwindSafe for ZalsaLocal { }
    };
}

impl_458!()