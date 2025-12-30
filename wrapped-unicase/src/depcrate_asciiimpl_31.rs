// Generated macro for impl_31 (impl)
macro_rules! Depcrate_asciiimpl_31 {
() => {
// Module: crate::ascii
// Provides: {"impl_31"}
// Dependencies: {}
impl < S : FromStr > FromStr for Ascii < S > { type Err = < S as FromStr > :: Err ; fn from_str (s : & str) -> Result < Ascii < S > , < S as FromStr > :: Err > { s . parse () . map (Ascii) } }
};
}
