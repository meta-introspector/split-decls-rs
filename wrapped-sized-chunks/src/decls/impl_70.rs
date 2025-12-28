macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < A , const N : usize > BorrowMut < [A] > for Chunk < A , N > { fn borrow_mut (& mut self) -> & mut [A] { self . as_mut_slice () } }
    };
}

impl_70!();