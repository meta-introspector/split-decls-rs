macro_rules! deps {
    () => {
        Array!();
        TinyVecIterator!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < A : Array > Iterator for TinyVecIterator < A > { type Item = A :: Item ; impl_mirrored ! { type Mirror = TinyVecIterator ; # [inline] fn next (self : & mut Self) -> Option < Self :: Item >; # [inline (always)] # [must_use] fn size_hint (self : & Self) -> (usize , Option < usize >) ; # [inline (always)] fn count (self : Self) -> usize ; # [inline] fn last (self : Self) -> Option < Self :: Item >; # [inline] fn nth (self : & mut Self , n : usize) -> Option < A :: Item >; } }
    };
}

impl_157!();