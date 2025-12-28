macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < A , const N : usize > Drop for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn drop (& mut self) { if mem :: needs_drop :: < A > () { let bits = self . map ; for index in & bits { unsafe { ptr :: drop_in_place (& mut self . values_mut () [index]) } } } } }
    };
}

impl_102!();