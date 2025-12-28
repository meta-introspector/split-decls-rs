macro_rules! deps {
    () => {
        DeArray!();
        IntoIter!();
        DeValue!();
        Iter!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a , 'i > IntoIterator for & 'a DeArray < 'i > { type Item = & 'a Spanned < DeValue < 'i > > ; type IntoIter = core :: slice :: Iter < 'a , Spanned < DeValue < 'i > > > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_190!();