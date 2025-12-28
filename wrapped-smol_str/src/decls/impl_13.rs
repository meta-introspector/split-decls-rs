macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl PartialEq < SmolStr > for String { # [inline (always)] fn eq (& self , other : & SmolStr) -> bool { other == self } }
    };
}

impl_13!();