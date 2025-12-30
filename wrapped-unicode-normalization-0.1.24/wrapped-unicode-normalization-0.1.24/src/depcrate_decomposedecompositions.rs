// Generated macro for Decompositions (struct)
macro_rules! Depcrate_decomposeDecompositions {
() => {
// Module: crate::decompose
// Provides: {"Decompositions"}
// Dependencies: {}
# [doc = " External iterator for a string decomposition's characters."] # [derive (Clone)] pub struct Decompositions < I > { kind : DecompositionType , iter : Fuse < I > , buffer : TinyVec < [(u8 , char) ; 4] > , ready : Range < usize > , }
};
}
