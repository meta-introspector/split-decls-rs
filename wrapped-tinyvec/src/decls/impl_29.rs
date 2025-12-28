macro_rules! deps {
    () => {
        ArrayVecSplice!();
        Array!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'p , A : Array , I : Iterator < Item = A :: Item > > Drop for ArrayVecSplice < 'p , A , I > { # [inline] fn drop (& mut self) { for _ in self . by_ref () { } for replacement in self . replacement . by_ref () { self . parent . insert (self . removal_end , replacement) ; self . removal_end += 1 ; } } }
    };
}

impl_29!();