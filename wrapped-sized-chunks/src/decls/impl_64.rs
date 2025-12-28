macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < A , const N : usize > Ord for Chunk < A , N > where A : Ord , { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
    };
}

impl_64!()