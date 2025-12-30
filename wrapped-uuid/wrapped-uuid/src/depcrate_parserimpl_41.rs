// Generated macro for impl_41 (impl)
macro_rules! Depcrate_parserimpl_41 {
() => {
// Module: crate::parser
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < String > for Uuid { type Error = Error ; fn try_from (uuid_str : String) -> Result < Self , Self :: Error > { Uuid :: try_from (uuid_str . as_ref ()) } }
};
}
