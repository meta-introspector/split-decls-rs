macro_rules! deps {
    () => {
        Binder!();
        Ty!();
        Interner!();
    };
}

macro_rules! UnsafeBinderInner {
    () => {
        deps!();
        # [derive_where (Clone , Copy , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct UnsafeBinderInner < I : Interner > (ty :: Binder < I , I :: Ty >) ;
    };
}

UnsafeBinderInner!();