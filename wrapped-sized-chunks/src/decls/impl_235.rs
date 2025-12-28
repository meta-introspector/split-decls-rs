macro_rules! deps {
    () => {
        RingBuffer!();
        IterMut!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > IntoIterator for & 'a mut RingBuffer < A , N > { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_235!()