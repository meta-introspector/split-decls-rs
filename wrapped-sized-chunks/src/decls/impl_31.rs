macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , A , T > IntoIterator for & 'a InlineArray < A , T > { type Item = & 'a A ; type IntoIter = SliceIter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_31!()