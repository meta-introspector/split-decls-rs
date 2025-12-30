// Generated macro for Recompositions (struct)
macro_rules! Depcrate_recomposeRecompositions {
() => {
// Module: crate::recompose
// Provides: {"Recompositions"}
// Dependencies: {}
# [doc = " External iterator for a string recomposition's characters."] # [derive (Clone)] pub struct Recompositions < I > { iter : Decompositions < I > , state : RecompositionState , buffer : TinyVec < [char ; 4] > , composee : Option < char > , last_ccc : Option < u8 > , }
};
}
