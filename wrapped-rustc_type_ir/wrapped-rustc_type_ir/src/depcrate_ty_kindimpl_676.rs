// Generated macro for impl_676 (impl)
macro_rules! Depcrate_ty_kindimpl_676 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_676"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < I : Interner , D : rustc_serialize :: Decoder > rustc_serialize :: Decodable < D > for UnsafeBinderInner < I > where I :: Ty : TypeVisitable < I > + rustc_serialize :: Decodable < D > , I :: BoundVarKinds : rustc_serialize :: Decodable < D > , { fn decode (decoder : & mut D) -> Self { let bound_vars = rustc_serialize :: Decodable :: decode (decoder) ; UnsafeBinderInner (ty :: Binder :: bind_with_vars (rustc_serialize :: Decodable :: decode (decoder) , bound_vars ,)) } }
};
}
