macro_rules! deps {
    () => {
        Slice!();
        Iter!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > IntoIterator for & 'a Slice < 'a , A , N > { type Item = & 'a A ; type IntoIter = Iter < 'a , A , N > ; # [inline] # [must_use] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_183!()