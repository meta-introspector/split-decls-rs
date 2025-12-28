macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a , S1 : AsRef < str > > PartialEq < Ascii < S1 > > for & 'a str { # [inline] fn eq (& self , other : & Ascii < S1 >) -> bool { other == self } }
    };
}

impl_11!();