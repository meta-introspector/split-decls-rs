macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'a , A : PartialOrd + 'a , const N : usize > PartialOrd for SliceMut < 'a , A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
    };
}

impl_198!();