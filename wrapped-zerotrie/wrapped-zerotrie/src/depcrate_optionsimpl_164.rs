// Generated macro for impl_164 (impl)
macro_rules! Depcrate_optionsimpl_164 {
() => {
// Module: crate::options
// Provides: {"impl_164"}
// Dependencies: {}
# [doc = " Branch nodes could be either binary search or PHF."] impl < S : ? Sized > ZeroTrieWithOptions for crate :: ZeroTriePerfectHash < S > { const OPTIONS : ZeroTrieBuilderOptions = ZeroTrieBuilderOptions { phf_mode : PhfMode :: UsePhf , ascii_mode : AsciiMode :: BinarySpans , capacity_mode : CapacityMode :: Normal , case_sensitivity : CaseSensitivity :: Sensitive , } ; }
};
}
