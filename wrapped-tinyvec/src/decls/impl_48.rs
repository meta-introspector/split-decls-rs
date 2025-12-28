macro_rules! deps {
    () => {
        Array!();
        ArrayVecIterator!();
        ArrayVec!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < A : Array > IntoIterator for ArrayVec < A > { type Item = A :: Item ; type IntoIter = ArrayVecIterator < A > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { ArrayVecIterator { base : 0 , tail : self . len , data : self . data } } }
    };
}

impl_48!();