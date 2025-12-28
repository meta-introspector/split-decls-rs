macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < A , const N : usize > Debug for SparseChunk < A , N > where A : Debug , BitsImpl < N > : Bits , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("SparseChunk") ? ; f . debug_map () . entries (self . entries ()) . finish () } }
    };
}

impl_114!()