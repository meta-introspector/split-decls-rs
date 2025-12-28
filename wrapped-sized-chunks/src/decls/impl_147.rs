macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > DoubleEndedIterator for Iter < 'a , A , N > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; Some (unsafe { & * self . buffer . ptr (self . right_index . dec ()) }) } } }
    };
}

impl_147!();