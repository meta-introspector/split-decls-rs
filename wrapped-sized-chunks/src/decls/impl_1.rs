macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < A , T > Iterator for Iter < A , T > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . array . remove (0) } fn size_hint (& self) -> (usize , Option < usize >) { (self . array . len () , Some (self . array . len ())) } }
    };
}

impl_1!();