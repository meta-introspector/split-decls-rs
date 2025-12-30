// Generated macro for impl_163 (impl)
macro_rules! Depcrate_optionsimpl_163 {
() => {
// Module: crate::options
// Provides: {"impl_163"}
// Dependencies: {}
# [doc = " All branch nodes are binary search"] # [doc = " and nodes use case-insensitive matching."] impl < S : ? Sized > ZeroTrieWithOptions for crate :: ZeroAsciiIgnoreCaseTrie < S > { const OPTIONS : ZeroTrieBuilderOptions = ZeroTrieBuilderOptions { phf_mode : PhfMode :: BinaryOnly , ascii_mode : AsciiMode :: AsciiOnly , capacity_mode : CapacityMode :: Normal , case_sensitivity : CaseSensitivity :: IgnoreCase , } ; }
};
}
