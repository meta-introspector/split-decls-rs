macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < str > for Key { # [inline] fn borrow (& self) -> & str { self . get () } }
    };
}

impl_130!()