macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_631!();