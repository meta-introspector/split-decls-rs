macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : AsRef < str > > PartialOrd for Unicode < T > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_25!();