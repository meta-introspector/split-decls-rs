macro_rules! deps {
    () => {
        SliceMut!();
        Iter!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < 'a , 'b , A : 'a , const N : usize > IntoIterator for & 'a SliceMut < 'a , A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_202!();