macro_rules! deps {
    () => {
        Array!();
        TinyVecIterator!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < A : Array > TinyVecIterator < A > { impl_mirrored ! { type Mirror = TinyVecIterator ; # [doc = " Returns the remaining items of this iterator as a slice."] # [inline] # [must_use] pub fn as_slice (self : & Self) -> & [A :: Item] ; } }
    };
}

impl_155!()