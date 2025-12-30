// Generated macro for impl_100 (impl)
macro_rules! Depcrate_ixdtfimpl_100 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_100"}
// Dependencies: {}
impl FromStr for DateTime < Iso > { type Err = ParseError ; fn from_str (rfc_9557_str : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (rfc_9557_str , Iso) } }
};
}
