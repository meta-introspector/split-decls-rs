// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < Vec < u8 > > for Signature { type Error = < [u8 ; SIGNATURE_BYTES] as TryFrom < Vec < u8 > > > :: Error ; # [inline] fn try_from (signature : Vec < u8 >) -> Result < Self , Self :: Error > { < [u8 ; SIGNATURE_BYTES] > :: try_from (signature) . map (Self :: from) } }
};
}
