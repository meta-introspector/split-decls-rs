macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > DoubleEndedIterator for IterMut < 'a , A , N > where A : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; let index = self . right_index . dec () ; Some (unsafe { & mut * self . mut_ptr (index) }) } } }
    };
}

impl_153!();