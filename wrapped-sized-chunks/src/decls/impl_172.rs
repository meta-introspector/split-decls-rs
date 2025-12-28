macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Clone for Slice < 'a , A , N > { # [inline] # [must_use] fn clone (& self) -> Self { Slice { buffer : self . buffer , range : self . range . clone () , } } }
    };
}

impl_172!()