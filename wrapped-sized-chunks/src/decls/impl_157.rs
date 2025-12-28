macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Iterator for Drain < 'a , A , N > { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . buffer . pop_front () } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . buffer . len () , Some (self . buffer . len ())) } }
    };
}

impl_157!();