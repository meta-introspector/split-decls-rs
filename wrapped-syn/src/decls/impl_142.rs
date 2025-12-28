macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Fields { type Item = & 'a Field ; type IntoIter = punctuated :: Iter < 'a , Field > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_142!()