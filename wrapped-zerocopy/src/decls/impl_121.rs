macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Display for BigEndian { # [inline] fn fmt (& self , _ : & mut Formatter < '_ >) -> fmt :: Result { match * self { } } }
    };
}

impl_121!();