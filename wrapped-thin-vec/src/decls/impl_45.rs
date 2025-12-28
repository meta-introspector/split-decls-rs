macro_rules! deps {
    () => {
        ThinVec!();
        IntoIter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > IntoIterator for ThinVec < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { vec : self , start : 0 , } } }
    };
}

impl_45!()