macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > AsMut < T > for Spanned < T > { fn as_mut (& mut self) -> & mut T { self . get_mut () } }
    };
}

impl_13!()