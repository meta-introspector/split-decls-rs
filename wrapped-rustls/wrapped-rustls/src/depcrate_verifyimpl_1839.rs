// Generated macro for impl_1839 (impl)
macro_rules! Depcrate_verifyimpl_1839 {
() => {
// Module: crate::verify
// Provides: {"impl_1839"}
// Dependencies: {}
impl DigitallySignedStruct { pub (crate) fn new (scheme : SignatureScheme , sig : Vec < u8 >) -> Self { Self { scheme , sig : PayloadU16 :: new (sig) , } } # [doc = " Get the signature."] pub fn signature (& self) -> & [u8] { & self . sig . 0 } }
};
}
