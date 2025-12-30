// Generated macro for impl_58 (impl)
macro_rules! Depcrate_traitsimpl_58 {
() => {
// Module: crate::traits
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > DecodePublicKey for T where T : for < 'a > TryFrom < SubjectPublicKeyInfoRef < 'a > , Error = Error > , { fn from_public_key_der (bytes : & [u8]) -> Result < Self > { Self :: try_from (SubjectPublicKeyInfoRef :: try_from (bytes) ?) } }
};
}
