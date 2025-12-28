macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > HasLength for SliceMut < 'a , A , N > { # [doc = " Get the length of the slice."] # [inline] # [must_use] fn len (& self) -> usize { self . range . end - self . range . start } }
    };
}

impl_185!();