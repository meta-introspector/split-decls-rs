// Generated macro for impl_171 (impl)
macro_rules! Depcrateimpl_171 {
() => {
// Module: crate
// Provides: {"impl_171"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < u8 > > for Uuid { type Error = Error ; fn try_from (value : std :: vec :: Vec < u8 >) -> Result < Self , Self :: Error > { Uuid :: from_slice (& value) } }
};
}
