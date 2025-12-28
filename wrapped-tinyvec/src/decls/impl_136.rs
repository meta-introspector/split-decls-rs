macro_rules! deps {
    () => {
        TinyVecDrain!();
        Array!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'p , A : Array > DoubleEndedIterator for TinyVecDrain < 'p , A > { impl_mirrored ! { type Mirror = TinyVecDrain ; # [inline] fn next_back (self : & mut Self) -> Option < Self :: Item >; # [inline] fn nth_back (self : & mut Self , n : usize) -> Option < Self :: Item >; } }
    };
}

impl_136!()