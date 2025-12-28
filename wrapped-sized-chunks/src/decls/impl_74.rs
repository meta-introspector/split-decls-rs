macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < A , const N : usize > DerefMut for Chunk < A , N > { fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
    };
}

impl_74!();