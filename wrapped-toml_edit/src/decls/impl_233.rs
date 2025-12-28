macro_rules! deps {
    () => {
        Table!();
        Item!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 's > IntoIterator for & 's Table { type Item = (& 's str , & 's Item) ; type IntoIter = Iter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_233!()