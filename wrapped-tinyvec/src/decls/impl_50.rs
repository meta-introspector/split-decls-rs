macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'a , A : Array > IntoIterator for & 'a ArrayVec < A > { type Item = & 'a A :: Item ; type IntoIter = core :: slice :: Iter < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_50!()