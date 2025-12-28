macro_rules! deps {
    () => {
        IntoIter!();
        Entry!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some ((key , entry)) = self . entries . next_back () { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } }
    };
}

impl_32!()