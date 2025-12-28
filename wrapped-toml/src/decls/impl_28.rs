macro_rules! deps {
    () => {
        Map!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , K , V > IntoIterator for & 'a Map < K , V > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { Iter { iter : self . map . iter () , } } }
    };
}

impl_28!()