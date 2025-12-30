// Generated macro for impl_123 (impl)
macro_rules! Depcrate_commonimpl_123 {
() => {
// Module: crate::common
// Provides: {"impl_123"}
// Dependencies: {}
impl FromStr for Codepoints { type Err = Error ; fn from_str (s : & str) -> Result < Codepoints , Error > { if s . contains ("..") { CodepointRange :: from_str (s) . map (Codepoints :: Range) } else { Codepoint :: from_str (s) . map (Codepoints :: Single) } } }
};
}
