macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : AsRef < str > > PartialOrd for Ascii < T > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_6!()