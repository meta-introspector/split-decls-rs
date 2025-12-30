// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl < S : FromStr + AsRef < str > > FromStr for UniCase < S > { type Err = < S as FromStr > :: Err ; fn from_str (s : & str) -> Result < UniCase < S > , Self :: Err > { s . parse () . map (UniCase :: new) } }
};
}
