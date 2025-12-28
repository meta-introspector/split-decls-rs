macro_rules! deps {
    () => {
        Array!();
        TinyVecIterator!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < A : Array > DoubleEndedIterator for TinyVecIterator < A > { impl_mirrored ! { type Mirror = TinyVecIterator ; # [inline] fn next_back (self : & mut Self) -> Option < Self :: Item >; # [inline] fn nth_back (self : & mut Self , n : usize) -> Option < Self :: Item >; } }
    };
}

impl_158!()