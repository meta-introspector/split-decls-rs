// Generated macro for impl_40 (impl)
macro_rules! Depcrate_parserimpl_40 {
() => {
// Module: crate::parser
// Provides: {"impl_40"}
// Dependencies: {}
impl TryFrom < & '_ str > for Uuid { type Error = Error ; fn try_from (uuid_str : & '_ str) -> Result < Self , Self :: Error > { Uuid :: parse_str (uuid_str) } }
};
}
