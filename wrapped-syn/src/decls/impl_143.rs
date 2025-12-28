macro_rules! deps {
    () => {
        IntoIter!();
        IterMut!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a mut Fields { type Item = & 'a mut Field ; type IntoIter = punctuated :: IterMut < 'a , Field > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_143!();