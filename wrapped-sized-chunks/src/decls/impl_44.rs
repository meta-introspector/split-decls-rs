macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > DoubleEndedIterator for Drain < 'a , A , N > where A : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_back ()) } } }
    };
}

impl_44!()