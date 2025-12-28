macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl PartialEq < SmolStr > for & str { # [inline (always)] fn eq (& self , other : & SmolStr) -> bool { * self == other } }
    };
}

impl_11!();