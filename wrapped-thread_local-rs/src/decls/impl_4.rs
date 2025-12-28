macro_rules! deps {
    () => {
        CachedIterMut!();
        CachedThreadLocal!();
        IntoIter!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a , T : Send + 'a > IntoIterator for & 'a mut CachedThreadLocal < T > { type Item = & 'a mut T ; type IntoIter = CachedIterMut < 'a , T > ; fn into_iter (self) -> CachedIterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_4!();