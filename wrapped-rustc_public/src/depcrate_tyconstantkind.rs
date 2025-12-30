// Generated macro for ConstantKind (enum)
macro_rules! Depcrate_tyConstantKind {
() => {
// Module: crate::ty
// Provides: {"ConstantKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum ConstantKind { Ty (TyConst) , Allocated (Allocation) , Unevaluated (UnevaluatedConst) , Param (ParamConst) , # [doc = " Store ZST constants."] # [doc = " We have to special handle these constants since its type might be generic."] ZeroSized , }
};
}
