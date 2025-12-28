macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < 'a , A : Ord + 'a , const N : usize > Ord for SliceMut < 'a , A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
    };
}

impl_199!()