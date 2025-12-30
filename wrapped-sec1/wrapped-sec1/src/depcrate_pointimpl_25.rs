// Generated macro for impl_25 (impl)
macro_rules! Depcrate_pointimpl_25 {
() => {
// Module: crate::point
// Provides: {"impl_25"}
// Dependencies: {}
impl < Size : ModulusSize > TryFrom < & [u8] > for EncodedPoint < Size > where Size : ModulusSize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_bytes (bytes) } }
};
}
