// Generated macro for impl_177 (impl)
macro_rules! Depcrate_verifying_keyimpl_177 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_177"}
// Dependencies: {}
impl < P : ParameterSet > TryFrom < & [u8] > for VerifyingKey < P > { type Error = Error ; # [allow (deprecated)] fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != P :: N :: USIZE * 2 { return Err (Error :: new ()) ; } let pk_seed = PkSeed (Array :: clone_from_slice (& bytes [.. P :: N :: USIZE])) ; let pk_root = Array :: clone_from_slice (& bytes [P :: N :: USIZE ..]) ; Ok (VerifyingKey { pk_seed , pk_root }) } }
};
}
