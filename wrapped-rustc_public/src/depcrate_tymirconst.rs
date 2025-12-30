// Generated macro for MirConst (struct)
macro_rules! Depcrate_tyMirConst {
() => {
// Module: crate::ty
// Provides: {"MirConst"}
// Dependencies: {}
# [doc = " Represents a constant in MIR"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct MirConst { # [doc = " The constant kind."] pub (crate) kind : ConstantKind , # [doc = " The constant type."] pub (crate) ty : Ty , # [doc = " Used for internal tracking of the internal constant."] pub id : MirConstId , }
};
}
