macro_rules! deps {
    () => {
        PCWSTR!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl AsRef < Self > for PCWSTR { fn as_ref (& self) -> & Self { self } }
    };
}

impl_114!();