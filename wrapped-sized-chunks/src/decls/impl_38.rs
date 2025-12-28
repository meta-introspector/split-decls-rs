macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < A , const N : usize > Iterator for Iter < A , N > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_front ()) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . chunk . len () , Some (self . chunk . len ())) } }
    };
}

impl_38!()