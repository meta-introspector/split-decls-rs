macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Iterator for Drain < A , N > where BitsImpl < N > : Bits , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . chunk . pop () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . chunk . len () ; (len , Some (len)) } }
    };
}

impl_88!();