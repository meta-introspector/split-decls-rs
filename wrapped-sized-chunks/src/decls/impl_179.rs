macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'a , A : PartialOrd + 'a , const N : usize > PartialOrd for Slice < 'a , A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
    };
}

impl_179!();