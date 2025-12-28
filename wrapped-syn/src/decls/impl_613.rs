macro_rules! deps {
    () => {
        Punctuated!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl < 'a , T , P > IntoIterator for & 'a Punctuated < T , P > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { Punctuated :: iter (self) } }
    };
}

impl_613!()