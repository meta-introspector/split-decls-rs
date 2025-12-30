// Generated macro for impl_521 (impl)
macro_rules! Depcrate_predicateimpl_521 {
() => {
// Module: crate::predicate
// Provides: {"impl_521"}
// Dependencies: {}
impl < I : Interner , U : Interner , A > Lift < U > for OutlivesPredicate < I , A > where A : Lift < U > , I :: Region : Lift < U , Lifted = U :: Region > , { type Lifted = OutlivesPredicate < U , A :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (OutlivesPredicate (self . 0 . lift_to_interner (cx) ? , self . 1 . lift_to_interner (cx) ?)) } }
};
}
