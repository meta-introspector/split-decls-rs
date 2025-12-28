macro_rules! deps {
    () => {
        IntoIter!();
        ThinVec!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a mut ThinVec < T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_47!();