macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < A , const N : usize > PartialOrd for Chunk < A , N > where A : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
    };
}

impl_63!()