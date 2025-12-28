macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'a , T > AsRef < [T] > for Drain < 'a , T > { fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_80!()