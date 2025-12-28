macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a , A : Array > IntoIterator for & 'a TinyVec < A > { type Item = & 'a A :: Item ; type IntoIter = core :: slice :: Iter < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_163!();