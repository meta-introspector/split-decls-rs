macro_rules! deps {
    () => {
        Drain!();
        Entry!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > Iterator for Drain < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { for entry in & mut self . inner { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some (v) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_43!()