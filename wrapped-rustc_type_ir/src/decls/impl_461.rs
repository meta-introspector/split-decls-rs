macro_rules! deps {
    () => {
        UnsafeBinderInner!();
        Interner!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < I : Interner > Eq for UnsafeBinderInner < I > { }
    };
}

impl_461!()