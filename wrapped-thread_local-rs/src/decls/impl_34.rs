macro_rules! deps {
    () => {
        ThreadLocal!();
        IterMut!();
        IntoIter!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a , T : Send > IntoIterator for & 'a mut ThreadLocal < T > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_34!()