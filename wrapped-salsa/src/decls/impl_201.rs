macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < T > Lookup < T > for T { fn into_owned (self) -> T { self } }
    };
}

impl_201!();