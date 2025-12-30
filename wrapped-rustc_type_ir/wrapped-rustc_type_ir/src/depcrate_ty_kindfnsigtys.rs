// Generated macro for FnSigTys (struct)
macro_rules! Depcrate_ty_kindFnSigTys {
() => {
// Module: crate::ty_kind
// Provides: {"FnSigTys"}
// Dependencies: {}
# [derive_where (Clone , Copy , Debug , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct FnSigTys < I : Interner > { pub inputs_and_output : I :: Tys , }
};
}
