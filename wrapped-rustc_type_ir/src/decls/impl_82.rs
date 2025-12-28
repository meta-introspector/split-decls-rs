macro_rules! deps {
    () => {
        SliceLike!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'a , T : Copy , const N : usize > SliceLike for & 'a [T ; N] { type Item = T ; type IntoIter = std :: iter :: Copied < std :: slice :: Iter < 'a , T > > ; fn iter (self) -> Self :: IntoIter { self . into_iter () . copied () } fn as_slice (& self) -> & [Self :: Item] { * self } }
    };
}

impl_82!();