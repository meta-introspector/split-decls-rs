macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Borrow < str > for SmolStr { # [inline (always)] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_41!()