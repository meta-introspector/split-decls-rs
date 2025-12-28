macro_rules! deps {
    () => {
        UnsafeBinderInner!();
        Binder!();
        Interner!();
        Ty!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < I : Interner > Deref for UnsafeBinderInner < I > { type Target = ty :: Binder < I , I :: Ty > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_465!();