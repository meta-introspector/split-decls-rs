// Generated macro for PhfMode (enum)
macro_rules! Depcrate_optionsPhfMode {
() => {
// Module: crate::options
// Provides: {"PhfMode"}
// Dependencies: {}
# [doc = " Whether to use the perfect hash function in the ZeroTrie."] # [derive (Copy , Clone)] pub (crate) enum PhfMode { # [doc = " Use binary search for all branch nodes."] BinaryOnly , # [doc = " Use the perfect hash function for large branch nodes."] UsePhf , }
};
}
