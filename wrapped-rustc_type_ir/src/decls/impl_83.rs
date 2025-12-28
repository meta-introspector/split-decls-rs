macro_rules! deps {
    () => {
        SliceLike!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < 'a , S : SliceLike > SliceLike for & 'a S { type Item = S :: Item ; type IntoIter = S :: IntoIter ; fn iter (self) -> Self :: IntoIter { (* self) . iter () } fn as_slice (& self) -> & [Self :: Item] { (* self) . as_slice () } }
    };
}

impl_83!()