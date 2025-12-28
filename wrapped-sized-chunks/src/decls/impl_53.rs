macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < A , const N : usize > Drop for Chunk < A , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . as_mut_slice ()) } } }
    };
}

impl_53!()