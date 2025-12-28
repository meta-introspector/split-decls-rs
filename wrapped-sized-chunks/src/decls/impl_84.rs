macro_rules! deps {
    () => {
        SparseChunk!();
        Iter!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Iterator for Iter < 'a , A , N > where BitsImpl < N > : Bits , { type Item = & 'a A ; fn next (& mut self) -> Option < Self :: Item > { self . indices . next () . map (| index | & self . chunk . values () [index]) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (SparseChunk :: < A , N > :: CAPACITY)) } }
    };
}

impl_84!();