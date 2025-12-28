macro_rules! deps {
    () => {
        Entry!();
        IntoIter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > Iterator for IntoIter < T > { type Item = (usize , T) ; fn next (& mut self) -> Option < Self :: Item > { for (key , entry) in & mut self . entries { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_31!()