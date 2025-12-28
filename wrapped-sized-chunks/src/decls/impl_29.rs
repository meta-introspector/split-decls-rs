macro_rules! deps {
    () => {
        Iter!();
        InlineArray!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < A , T > IntoIterator for InlineArray < A , T > { type Item = A ; type IntoIter = Iter < A , T > ; fn into_iter (self) -> Self :: IntoIter { Iter { array : self } } }
    };
}

impl_29!()