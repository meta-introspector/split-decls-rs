macro_rules! deps {
    () => {
        ArrayVec!();
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T , A > From < & '_ [T] > for TinyVec < A > where T : Clone + Default , A : Array < Item = T > , { # [inline] fn from (slice : & [T]) -> Self { if let Ok (arr) = ArrayVec :: try_from (slice) { TinyVec :: Inline (arr) } else { TinyVec :: Heap (slice . into ()) } } }
    };
}

impl_150!()