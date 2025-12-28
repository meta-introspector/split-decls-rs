macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , A , T > IntoIterator for & 'a mut InlineArray < A , T > { type Item = & 'a mut A ; type IntoIter = SliceIterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_32!();