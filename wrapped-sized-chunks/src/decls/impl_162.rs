macro_rules! deps {
    () => {
        OwnedIter!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < A , const N : usize > Iterator for OwnedIter < A , N > { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . buffer . pop_front () } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . buffer . len () , Some (self . buffer . len ())) } }
    };
}

impl_162!();