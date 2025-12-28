macro_rules! deps {
    () => {
        Entry!();
        Iter!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for Iter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some ((key , entry)) = self . entries . next_back () { if let Entry :: Occupied (ref v) = * entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } }
    };
}

impl_42!()