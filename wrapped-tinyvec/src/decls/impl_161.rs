macro_rules! deps {
    () => {
        TinyVec!();
        TinyVecIterator!();
        Array!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < A : Array > IntoIterator for TinyVec < A > { type Item = A :: Item ; type IntoIter = TinyVecIterator < A > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { match self { TinyVec :: Inline (a) => TinyVecIterator :: Inline (a . into_iter ()) , TinyVec :: Heap (v) => TinyVecIterator :: Heap (v . into_iter ()) , } } }
    };
}

impl_161!();