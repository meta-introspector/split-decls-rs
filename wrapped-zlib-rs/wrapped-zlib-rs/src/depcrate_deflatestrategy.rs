// Generated macro for Strategy (enum)
macro_rules! Depcrate_deflateStrategy {
() => {
// Module: crate::deflate
// Provides: {"Strategy"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] # [cfg_attr (feature = "__internal-fuzz" , derive (arbitrary :: Arbitrary))] pub enum Strategy { # [default] Default = 0 , Filtered = 1 , HuffmanOnly = 2 , Rle = 3 , Fixed = 4 , }
};
}
