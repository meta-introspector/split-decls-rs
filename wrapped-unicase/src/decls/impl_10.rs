macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < S1 : AsRef < str > > PartialEq < Ascii < S1 > > for String { # [inline] fn eq (& self , other : & Ascii < S1 >) -> bool { other == self } }
    };
}

impl_10!();