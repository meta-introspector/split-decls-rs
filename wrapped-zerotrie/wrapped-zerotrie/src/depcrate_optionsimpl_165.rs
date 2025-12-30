// Generated macro for impl_165 (impl)
macro_rules! Depcrate_optionsimpl_165 {
() => {
// Module: crate::options
// Provides: {"impl_165"}
// Dependencies: {}
# [doc = " No limited capacity assertion."] impl < S : ? Sized > ZeroTrieWithOptions for crate :: ZeroTrieExtendedCapacity < S > { const OPTIONS : ZeroTrieBuilderOptions = ZeroTrieBuilderOptions { phf_mode : PhfMode :: UsePhf , ascii_mode : AsciiMode :: BinarySpans , capacity_mode : CapacityMode :: Extended , case_sensitivity : CaseSensitivity :: Sensitive , } ; }
};
}
