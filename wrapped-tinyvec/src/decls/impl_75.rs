macro_rules! deps {
    () => {
        ArrayVecDrain!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , T : 'a + Default > DoubleEndedIterator for ArrayVecDrain < 'a , T > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (core :: mem :: take) } # [inline] fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth_back (n) . map (core :: mem :: take) } }
    };
}

impl_75!();