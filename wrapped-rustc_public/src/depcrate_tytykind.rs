// Generated macro for TyKind (enum)
macro_rules! Depcrate_tyTyKind {
() => {
// Module: crate::ty
// Provides: {"TyKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TyKind { RigidTy (RigidTy) , Alias (AliasKind , AliasTy) , Param (ParamTy) , Bound (usize , BoundTy) , }
};
}
