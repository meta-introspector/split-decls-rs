macro_rules! deps {
    () => {
        Chunk!();
        InlineArray!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < A , T , const N : usize > From < InlineArray < A , T > > for Chunk < A , N > { # [inline] fn from (mut array : InlineArray < A , T >) -> Self { Self :: from (& mut array) } }
    };
}

impl_67!()