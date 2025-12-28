macro_rules! deps {
    () => {
        Slice!();
        SliceMut!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Into < Slice < 'a , A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn into (self) -> Slice < 'a , A , N > { self . unmut () } }
    };
}

impl_190!()