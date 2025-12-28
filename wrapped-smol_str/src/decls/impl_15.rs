macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl PartialEq < SmolStr > for & String { # [inline (always)] fn eq (& self , other : & SmolStr) -> bool { * self == other } }
    };
}

impl_15!()