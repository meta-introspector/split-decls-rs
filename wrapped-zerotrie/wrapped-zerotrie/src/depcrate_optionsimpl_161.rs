// Generated macro for impl_161 (impl)
macro_rules! Depcrate_optionsimpl_161 {
() => {
// Module: crate::options
// Provides: {"impl_161"}
// Dependencies: {}
# [doc = " All branch nodes are binary search"] # [doc = " and there are no span nodes."] impl < S : ? Sized > ZeroTrieWithOptions for crate :: ZeroTrieSimpleAscii < S > { const OPTIONS : ZeroTrieBuilderOptions = ZeroTrieBuilderOptions { phf_mode : PhfMode :: BinaryOnly , ascii_mode : AsciiMode :: AsciiOnly , capacity_mode : CapacityMode :: Normal , case_sensitivity : CaseSensitivity :: Sensitive , } ; }
};
}
