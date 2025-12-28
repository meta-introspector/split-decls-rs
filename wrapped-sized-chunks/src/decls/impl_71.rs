macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < A , const N : usize > AsRef < [A] > for Chunk < A , N > { fn as_ref (& self) -> & [A] { self . as_slice () } }
    };
}

impl_71!();