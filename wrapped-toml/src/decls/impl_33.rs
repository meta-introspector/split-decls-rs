macro_rules! deps {
    () => {
        Map!();
        IntoIter!();
        IterMut!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , K , V > IntoIterator for & 'a mut Map < K , V > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IterMut { iter : self . map . iter_mut () , } } }
    };
}

impl_33!()