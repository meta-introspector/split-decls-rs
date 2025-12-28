macro_rules! deps {
    () => {
        IterMut!();
        IntoIter!();
        Punctuated!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < 'a , T , P > IntoIterator for & 'a mut Punctuated < T , P > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { Punctuated :: iter_mut (self) } }
    };
}

impl_614!()