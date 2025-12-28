macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > Borrow < str > for Label < 'a > { # [inline (always)] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_21!();