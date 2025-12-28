macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < A , const N : usize > Eq for Chunk < A , N > where A : Eq { }
    };
}

impl_62!()