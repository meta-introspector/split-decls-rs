macro_rules! deps {
    () => {
        SliceMut!();
        IterMut!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'a , 'b , A : 'a , const N : usize > IntoIterator for & 'a mut SliceMut < 'a , A , N > { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_203!()