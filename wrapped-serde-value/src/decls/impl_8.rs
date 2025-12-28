macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PartialOrd for Value { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }
    };
}

impl_8!()