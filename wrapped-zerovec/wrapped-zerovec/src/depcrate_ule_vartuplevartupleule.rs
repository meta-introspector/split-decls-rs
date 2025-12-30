// Generated macro for VarTupleULE (struct)
macro_rules! Depcrate_ule_vartupleVarTupleULE {
() => {
// Module: crate::ule::vartuple
// Provides: {"VarTupleULE"}
// Dependencies: {}
# [doc = " A dynamically-sized type combining a sized and an unsized type."] # [doc = ""] # [doc = " See the module for examples."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_structs)] # [repr (C)] pub struct VarTupleULE < A : AsULE , V : VarULE + ? Sized > { pub sized : A :: ULE , pub variable : V , }
};
}
