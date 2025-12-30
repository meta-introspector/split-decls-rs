// Generated macro for impl_675 (impl)
macro_rules! Depcrate_ty_kindimpl_675 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_675"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < I : Interner , E : rustc_serialize :: Encoder > rustc_serialize :: Encodable < E > for UnsafeBinderInner < I > where I :: Ty : rustc_serialize :: Encodable < E > , I :: BoundVarKinds : rustc_serialize :: Encodable < E > , { fn encode (& self , e : & mut E) { self . bound_vars () . encode (e) ; self . as_ref () . skip_binder () . encode (e) ; } }
};
}
