macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_641!();