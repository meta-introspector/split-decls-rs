// Generated macro for impl_667 (impl)
macro_rules! Depcrate_ty_kindimpl_667 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_667"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , FnSig < I > > { # [inline] pub fn inputs (self) -> ty :: Binder < I , I :: FnInputTys > { self . map_bound (| fn_sig | fn_sig . inputs ()) } # [inline] # [track_caller] pub fn input (self , index : usize) -> ty :: Binder < I , I :: Ty > { self . map_bound (| fn_sig | fn_sig . inputs () . get (index) . unwrap ()) } pub fn inputs_and_output (self) -> ty :: Binder < I , I :: Tys > { self . map_bound (| fn_sig | fn_sig . inputs_and_output) } # [inline] pub fn output (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| fn_sig | fn_sig . output ()) } pub fn c_variadic (self) -> bool { self . skip_binder () . c_variadic } pub fn safety (self) -> I :: Safety { self . skip_binder () . safety } pub fn abi (self) -> I :: Abi { self . skip_binder () . abi } pub fn is_fn_trait_compatible (& self) -> bool { self . skip_binder () . is_fn_trait_compatible () } pub fn split (self) -> (ty :: Binder < I , FnSigTys < I > > , FnHeader < I >) { let hdr = FnHeader { c_variadic : self . c_variadic () , safety : self . safety () , abi : self . abi () } ; (self . map_bound (| sig | FnSigTys { inputs_and_output : sig . inputs_and_output }) , hdr) } }
};
}
