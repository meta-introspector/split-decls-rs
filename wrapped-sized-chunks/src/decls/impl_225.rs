macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < A : Ord , const N : usize > Ord for RingBuffer < A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
    };
}

impl_225!()