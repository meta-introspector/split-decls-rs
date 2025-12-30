// Generated macro for impl_680 (impl)
macro_rules! Depcrate_ty_kindimpl_680 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_680"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , FnSigTys < I > > { pub fn with (self , hdr : FnHeader < I >) -> ty :: Binder < I , FnSig < I > > { self . map_bound (| sig_tys | FnSig { inputs_and_output : sig_tys . inputs_and_output , c_variadic : hdr . c_variadic , safety : hdr . safety , abi : hdr . abi , }) } # [inline] pub fn inputs (self) -> ty :: Binder < I , I :: FnInputTys > { self . map_bound (| sig_tys | sig_tys . inputs ()) } # [inline] # [track_caller] pub fn input (self , index : usize) -> ty :: Binder < I , I :: Ty > { self . map_bound (| sig_tys | sig_tys . inputs () . get (index) . unwrap ()) } pub fn inputs_and_output (self) -> ty :: Binder < I , I :: Tys > { self . map_bound (| sig_tys | sig_tys . inputs_and_output) } # [inline] pub fn output (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| sig_tys | sig_tys . output ()) } }
};
}
