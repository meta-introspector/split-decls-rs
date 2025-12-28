macro_rules! deps {
    () => {
        Iter!();
        RingBuffer!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > IntoIterator for & 'a RingBuffer < A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_234!();