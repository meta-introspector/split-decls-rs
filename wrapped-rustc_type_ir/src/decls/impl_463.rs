macro_rules! deps {
    () => {
        Interner!();
        Binder!();
        UnsafeBinderInner!();
        Ty!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < I : Interner > From < UnsafeBinderInner < I > > for ty :: Binder < I , I :: Ty > { fn from (value : UnsafeBinderInner < I >) -> Self { value . 0 } }
    };
}

impl_463!()