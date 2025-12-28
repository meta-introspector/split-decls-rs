macro_rules! deps {
    () => {
        BuildMetadata!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl PartialOrd for BuildMetadata { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }
    };
}

impl_55!()