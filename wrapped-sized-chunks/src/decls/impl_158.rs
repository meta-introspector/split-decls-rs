macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > DoubleEndedIterator for Drain < 'a , A , N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . buffer . pop_back () } }
    };
}

impl_158!()