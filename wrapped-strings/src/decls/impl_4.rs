macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Clone for BSTR { fn clone (& self) -> Self { Self :: from_wide (self) } }
    };
}

impl_4!();