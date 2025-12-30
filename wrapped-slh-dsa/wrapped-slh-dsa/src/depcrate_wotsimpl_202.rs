// Generated macro for impl_202 (impl)
macro_rules! Depcrate_wotsimpl_202 {
() => {
// Module: crate::wots
// Provides: {"impl_202"}
// Dependencies: {}
impl < P : WotsParams > TryFrom < & [u8] > for WotsSig < P > { type Error = () ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () != Self :: SIZE { return Err (()) ; } let mut sig = Array :: < Array < u8 , P :: N > , P :: WotsSigLen > :: default () ; for i in 0 .. P :: WotsSigLen :: USIZE { sig [i] . copy_from_slice (& value [i * P :: N :: USIZE .. (i + 1) * P :: N :: USIZE]) ; } Ok (WotsSig (sig)) } }
};
}
