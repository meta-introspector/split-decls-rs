macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a , A : Array > IntoIterator for & 'a mut ArrayVec < A > { type Item = & 'a mut A :: Item ; type IntoIter = core :: slice :: IterMut < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_49!();