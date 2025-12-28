macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > IntoIterator for & 'a Chunk < A , N > { type Item = & 'a A ; type IntoIter = SliceIter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_76!()