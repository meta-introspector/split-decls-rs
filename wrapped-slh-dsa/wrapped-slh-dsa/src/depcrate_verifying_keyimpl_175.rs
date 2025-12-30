// Generated macro for impl_175 (impl)
macro_rules! Depcrate_verifying_keyimpl_175 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_175"}
// Dependencies: {}
impl < P : ParameterSet > From < & VerifyingKey < P > > for Array < u8 , P :: VkLen > { fn from (vk : & VerifyingKey < P >) -> Array < u8 , P :: VkLen > { vk . to_bytes () } }
};
}
