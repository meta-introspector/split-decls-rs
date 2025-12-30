// Generated macro for impl_1433 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1433 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1433"}
// Dependencies: {}
impl < 'a > Codec < 'a > for Payload < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { bytes . extend_from_slice (self . bytes ()) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Ok (Self :: read (r)) } }
};
}
