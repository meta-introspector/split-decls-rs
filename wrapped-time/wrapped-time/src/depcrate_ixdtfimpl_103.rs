// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ixdtfimpl_103 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_103"}
// Dependencies: {}
impl FromStr for Time { type Err = ParseError ; fn from_str (rfc_9557_str : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (rfc_9557_str) } }
};
}
