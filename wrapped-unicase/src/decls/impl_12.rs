macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < S1 : AsRef < str > , S2 : AsRef < str > > PartialEq < S2 > for Ascii < S1 > { # [inline] fn eq (& self , other : & S2) -> bool { self . as_ref () . eq_ignore_ascii_case (other . as_ref ()) } }
    };
}

impl_12!()