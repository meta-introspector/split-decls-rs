macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_654 {
    () => {
        deps!();
        impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_654!();