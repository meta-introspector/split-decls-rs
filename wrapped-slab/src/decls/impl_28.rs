macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        Slab!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a Slab < T > { type Item = (usize , & 'a T) ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
    };
}

impl_28!()