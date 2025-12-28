macro_rules! deps {
    () => {
        IntoIter!();
        CachedIntoIter!();
        CachedThreadLocal!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T : Send > IntoIterator for CachedThreadLocal < T > { type Item = T ; type IntoIter = CachedIntoIter < T > ; fn into_iter (self) -> CachedIntoIter < T > { CachedIntoIter { inner : self . inner . into_iter () , } } }
    };
}

impl_3!()