macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl Borrow < [u8] > for OwnedSlice { # [inline] fn borrow (& self) -> & [u8] { self } }
    };
}

impl_345!()