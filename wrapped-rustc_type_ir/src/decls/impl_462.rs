macro_rules! deps {
    () => {
        Ty!();
        UnsafeBinderInner!();
        Binder!();
        Interner!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < I : Interner > From < ty :: Binder < I , I :: Ty > > for UnsafeBinderInner < I > { fn from (value : ty :: Binder < I , I :: Ty >) -> Self { UnsafeBinderInner (value) } }
    };
}

impl_462!();