macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl PartialOrd for DateTime { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_189!();