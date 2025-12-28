macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , 'b > PartialEq < Label < 'b > > for Label < 'a > { # [inline (always)] fn eq (& self , other : & Label < 'b >) -> bool { self . as_str () == other . as_str () } }
    };
}

impl_18!()