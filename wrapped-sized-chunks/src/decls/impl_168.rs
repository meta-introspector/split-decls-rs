macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > HasLength for Slice < 'a , A , N > { # [doc = " Get the length of the slice."] # [inline] # [must_use] fn len (& self) -> usize { self . range . end - self . range . start } }
    };
}

impl_168!()