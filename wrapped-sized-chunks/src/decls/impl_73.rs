macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < A , const N : usize > Deref for Chunk < A , N > { type Target = [A] ; fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_73!()