macro_rules! deps {
    () => {
        SetGlobalDefaultError!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl SetGlobalDefaultError { const MESSAGE : & 'static str = "a global default trace dispatcher has already been set" ; }
    };
}

impl_79!()