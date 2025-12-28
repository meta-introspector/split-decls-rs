macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < S : AsRef < str > > AsRef < str > for Ascii < S > { # [inline] fn as_ref (& self) -> & str { self . 0 . as_ref () } }
    };
}

impl_8!();