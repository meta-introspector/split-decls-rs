// Generated macro for impl_176 (impl)
macro_rules! Depcrate_verifying_keyimpl_176 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_176"}
// Dependencies: {}
impl < P : ParameterSet > From < Array < u8 , P :: VkLen > > for VerifyingKey < P > { # [allow (deprecated)] fn from (bytes : Array < u8 , P :: VkLen >) -> VerifyingKey < P > { debug_assert ! (P :: VkLen :: USIZE == 2 * P :: N :: USIZE) ; let pk_seed = PkSeed (Array :: clone_from_slice (& bytes [.. P :: N :: USIZE])) ; let pk_root = Array :: clone_from_slice (& bytes [P :: N :: USIZE ..]) ; VerifyingKey { pk_seed , pk_root } } }
};
}
