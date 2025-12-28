macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > IntoIterator for & 'a mut Chunk < A , N > { type Item = & 'a mut A ; type IntoIter = SliceIterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_77!()