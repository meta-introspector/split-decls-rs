macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Display for LittleEndian { # [inline] fn fmt (& self , _ : & mut Formatter < '_ >) -> fmt :: Result { match * self { } } }
    };
}

impl_124!()