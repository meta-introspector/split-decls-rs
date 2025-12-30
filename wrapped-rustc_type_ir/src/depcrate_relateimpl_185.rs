// Generated macro for impl_185 (impl)
macro_rules! Depcrate_relateimpl_185 {
() => {
// Module: crate::relate
// Provides: {"impl_185"}
// Dependencies: {}
impl < I : Interner , T : Relate < I > > Relate < I > for ty :: Binder < I , T > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: Binder < I , T > , b : ty :: Binder < I , T > ,) -> RelateResult < I , ty :: Binder < I , T > > { relation . binders (a , b) } }
};
}
