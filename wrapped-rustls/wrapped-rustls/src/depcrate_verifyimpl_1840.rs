// Generated macro for impl_1840 (impl)
macro_rules! Depcrate_verifyimpl_1840 {
() => {
// Module: crate::verify
// Provides: {"impl_1840"}
// Dependencies: {}
impl Codec < '_ > for DigitallySignedStruct { fn encode (& self , bytes : & mut Vec < u8 >) { self . scheme . encode (bytes) ; self . sig . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let scheme = SignatureScheme :: read (r) ? ; let sig = PayloadU16 :: read (r) ? ; Ok (Self { scheme , sig }) } }
};
}
