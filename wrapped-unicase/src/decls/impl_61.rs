macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : AsRef < str > > PartialOrd for UniCase < T > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_61!();