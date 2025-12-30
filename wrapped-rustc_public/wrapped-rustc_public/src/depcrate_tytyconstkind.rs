// Generated macro for TyConstKind (enum)
macro_rules! Depcrate_tyTyConstKind {
() => {
// Module: crate::ty
// Provides: {"TyConstKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum TyConstKind { Param (ParamConst) , Bound (DebruijnIndex , BoundVar) , Unevaluated (ConstDef , GenericArgs) , Value (Ty , Allocation) , ZSTValue (Ty) , }
};
}
