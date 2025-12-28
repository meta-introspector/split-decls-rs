macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl PartialOrd for Prerelease { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }
    };
}

impl_54!()