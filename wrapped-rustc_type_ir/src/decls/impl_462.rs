macro_rules! deps {
    () => {
        Binder!();
        Ty!();
        UnsafeBinderInner!();
        Interner!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < I : Interner > From < ty :: Binder < I , I :: Ty > > for UnsafeBinderInner < I > { fn from (value : ty :: Binder < I , I :: Ty >) -> Self { UnsafeBinderInner (value) } }
    };
}

impl_462!()