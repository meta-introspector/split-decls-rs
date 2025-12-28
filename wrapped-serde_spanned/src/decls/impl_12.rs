macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T > AsRef < T > for Spanned < T > { fn as_ref (& self) -> & T { self . get_ref () } }
    };
}

impl_12!()