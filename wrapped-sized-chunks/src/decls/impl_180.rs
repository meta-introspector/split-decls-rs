macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'a , A : Ord + 'a , const N : usize > Ord for Slice < 'a , A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
    };
}

impl_180!();