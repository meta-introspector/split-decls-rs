macro_rules! deps {
    () => {
        Iter!();
        Chunk!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < A , const N : usize > IntoIterator for Chunk < A , N > { type Item = A ; type IntoIter = Iter < A , N > ; fn into_iter (self) -> Self :: IntoIter { Iter { chunk : self } } }
    };
}

impl_80!();