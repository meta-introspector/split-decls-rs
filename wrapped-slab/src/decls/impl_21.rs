macro_rules! deps {
    () => {
        Slab!();
        IntoIter!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > IntoIterator for Slab < T > { type Item = (usize , T) ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { entries : self . entries . into_iter () . enumerate () , len : self . len , } } }
    };
}

impl_21!()