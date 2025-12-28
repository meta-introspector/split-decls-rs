macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl PartialEq for Index { # [inline (always)] fn eq (& self , other : & Self) -> bool { self . 0 == other . 0 } }
    };
}

impl_36!()