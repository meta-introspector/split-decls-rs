macro_rules! deps {
    () => {
        RingBuffer!();
        OwnedIter!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < A , const N : usize > IntoIterator for RingBuffer < A , N > { type Item = A ; type IntoIter = OwnedIter < A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { OwnedIter { buffer : self } } }
    };
}

impl_233!()