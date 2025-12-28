macro_rules! deps {
    () => {
        AsField!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl AsField for str { # [inline] fn as_field (& self , metadata : & Metadata < '_ >) -> Option < Field > { metadata . fields () . field (& self) } }
    };
}

impl_34!()