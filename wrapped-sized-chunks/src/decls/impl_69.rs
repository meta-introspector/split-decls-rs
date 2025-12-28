macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < A , const N : usize > Borrow < [A] > for Chunk < A , N > { fn borrow (& self) -> & [A] { self . as_slice () } }
    };
}

impl_69!();