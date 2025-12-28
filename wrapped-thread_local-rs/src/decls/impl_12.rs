macro_rules! deps {
    () => {
        ThreadLocal!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'a , T : Send + Sync > IntoIterator for & 'a ThreadLocal < T > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_12!()