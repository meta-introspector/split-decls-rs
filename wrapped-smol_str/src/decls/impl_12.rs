macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl PartialEq < String > for SmolStr { # [inline (always)] fn eq (& self , other : & String) -> bool { self . as_str () == other } }
    };
}

impl_12!();