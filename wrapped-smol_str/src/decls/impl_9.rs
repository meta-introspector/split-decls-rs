macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl PartialEq < SmolStr > for str { # [inline (always)] fn eq (& self , other : & SmolStr) -> bool { other == self } }
    };
}

impl_9!()