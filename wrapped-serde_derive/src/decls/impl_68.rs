macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl PartialOrd for Name { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (Ord :: cmp (self , other)) } }
    };
}

impl_68!();