macro_rules! deps {
    () => {
        RawIter!();
        ThreadLocal!();
        IntoIter!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T : Send > IntoIterator for ThreadLocal < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { thread_local : self , raw : RawIter :: new () , } } }
    };
}

impl_32!()