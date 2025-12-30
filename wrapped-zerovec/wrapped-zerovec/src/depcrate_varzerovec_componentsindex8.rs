// Generated macro for Index8 (struct)
macro_rules! Depcrate_varzerovec_componentsIndex8 {
() => {
// Module: crate::varzerovec::components
// Provides: {"Index8"}
// Dependencies: {}
# [doc = " This is a [`VarZeroVecFormat`] that stores u8s in the index array, and a u8 for a length."] # [doc = ""] # [doc = " Will have a smaller data size, but it's *extremely* likely for larger arrays"] # [doc = " to be unrepresentable (and error on construction). Should probably be used"] # [doc = " for known-small arrays, where all but the last field are known-small."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_structs)] pub struct Index8 ;
};
}
