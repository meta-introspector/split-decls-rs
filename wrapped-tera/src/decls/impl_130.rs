macro_rules! deps {
    () => {
        OrderedF64!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl PartialOrd for OrderedF64 { fn partial_cmp (& self , other : & OrderedF64) -> Option < Ordering > { Some (total_cmp (& self . 0 , & other . 0)) } }
    };
}

impl_130!();