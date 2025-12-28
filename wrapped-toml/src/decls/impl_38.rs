macro_rules! deps {
    () => {
        IntoIter!();
        Map!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < K , V > IntoIterator for Map < K , V > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { iter : self . map . into_iter () , } } }
    };
}

impl_38!()