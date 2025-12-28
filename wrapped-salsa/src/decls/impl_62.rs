macro_rules! deps {
    () => {
        CycleHeadsIterator!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl DoubleEndedIterator for CycleHeadsIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { loop { let next = self . inner . next_back () ? ; if next . removed . load (Ordering :: Relaxed) { continue ; } return Some (next) ; } } }
    };
}

impl_62!();