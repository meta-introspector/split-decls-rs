macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < A , const N : usize > AsMut < [A] > for Chunk < A , N > { fn as_mut (& mut self) -> & mut [A] { self . as_mut_slice () } }
    };
}

impl_72!()