macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl PartialOrd for Lifetime { fn partial_cmp (& self , other : & Lifetime) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_403!()