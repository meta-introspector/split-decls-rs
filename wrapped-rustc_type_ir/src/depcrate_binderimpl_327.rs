// Generated macro for impl_327 (impl)
macro_rules! Depcrate_binderimpl_327 {
() => {
// Module: crate::binder
// Provides: {"impl_327"}
// Dependencies: {}
impl < I : Interner , U : Interner , T > Lift < U > for Binder < I , T > where T : Lift < U > , I :: BoundVarKinds : Lift < U , Lifted = U :: BoundVarKinds > , { type Lifted = Binder < U , T :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (Binder { value : self . value . lift_to_interner (cx) ? , bound_vars : self . bound_vars . lift_to_interner (cx) ? , }) } }
};
}
