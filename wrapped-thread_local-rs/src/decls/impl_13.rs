macro_rules! deps {
    () => {
        IntoIter!();
        IterMut!();
        ThreadLocal!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a , T : Send > IntoIterator for & 'a mut ThreadLocal < T > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_13!()