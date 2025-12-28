macro_rules! deps {
    () => {
        SparseChunk!();
        OptionDrain!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Iterator for OptionDrain < A , N > where BitsImpl < N > : Bits , { type Item = Option < A > ; fn next (& mut self) -> Option < Self :: Item > { if self . index < N { let result = self . chunk . remove (self . index) ; self . index += 1 ; Some (result) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (SparseChunk :: < A , N > :: CAPACITY - self . index , Some (SparseChunk :: < A , N > :: CAPACITY - self . index) ,) } }
    };
}

impl_94!();