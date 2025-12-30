// Generated macro for InferConst (enum)
macro_rules! Depcrate_const_kindInferConst {
() => {
// Module: crate::const_kind
// Provides: {"InferConst"}
// Dependencies: {}
# [doc = " An inference variable for a const, for use in const generics."] # [derive (Copy , Clone , Eq , PartialEq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext))] pub enum InferConst { # [doc = " Infer the value of the const."] Var (ConstVid) , # [doc = " A fresh const variable. See `infer::freshen` for more details."] Fresh (u32) , }
};
}
