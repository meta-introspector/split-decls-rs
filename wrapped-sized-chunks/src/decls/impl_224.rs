macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < A : PartialOrd , const N : usize > PartialOrd for RingBuffer < A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
    };
}

impl_224!()