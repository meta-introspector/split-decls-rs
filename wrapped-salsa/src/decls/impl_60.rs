macro_rules! deps {
    () => {
        CycleHeadsIterator!();
        CycleHead!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < 'a > Iterator for CycleHeadsIterator < 'a > { type Item = & 'a CycleHead ; fn next (& mut self) -> Option < Self :: Item > { loop { let next = self . inner . next () ? ; if next . removed . load (Ordering :: Relaxed) { continue ; } return Some (next) ; } } }
    };
}

impl_60!()