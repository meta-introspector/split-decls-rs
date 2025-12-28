macro_rules! deps {
    () => {
        Drain!();
        Entry!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for Drain < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some (entry) = self . inner . next_back () { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some (v) ; } } debug_assert_eq ! (self . len , 0) ; None } }
    };
}

impl_50!()