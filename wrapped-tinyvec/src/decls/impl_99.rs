macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 's , T > IntoIterator for SliceVec < 's , T > { type Item = & 's mut T ; type IntoIter = core :: slice :: IterMut < 's , T > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . data . iter_mut () } }
    };
}

impl_99!()