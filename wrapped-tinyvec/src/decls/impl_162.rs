macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'a , A : Array > IntoIterator for & 'a mut TinyVec < A > { type Item = & 'a mut A :: Item ; type IntoIter = core :: slice :: IterMut < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_162!();