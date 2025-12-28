macro_rules! deps {
    () => {
        IntoIter!();
        ThinVec!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a ThinVec < T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
    };
}

impl_46!()