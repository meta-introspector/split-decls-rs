macro_rules! deps {
    () => {
        SliceLike!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'a , T : Copy > SliceLike for & 'a [T] { type Item = T ; type IntoIter = std :: iter :: Copied < std :: slice :: Iter < 'a , T > > ; fn iter (self) -> Self :: IntoIter { self . iter () . copied () } fn as_slice (& self) -> & [Self :: Item] { * self } }
    };
}

impl_81!()