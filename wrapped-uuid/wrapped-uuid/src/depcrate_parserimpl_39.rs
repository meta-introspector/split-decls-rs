// Generated macro for impl_39 (impl)
macro_rules! Depcrate_parserimpl_39 {
() => {
// Module: crate::parser
// Provides: {"impl_39"}
// Dependencies: {}
impl str :: FromStr for Uuid { type Err = Error ; fn from_str (uuid_str : & str) -> Result < Self , Self :: Err > { Uuid :: parse_str (uuid_str) } }
};
}
