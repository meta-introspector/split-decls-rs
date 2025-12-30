// Generated macro for DigitallySignedStruct (struct)
macro_rules! Depcrate_verifyDigitallySignedStruct {
() => {
// Module: crate::verify
// Provides: {"DigitallySignedStruct"}
// Dependencies: {}
# [doc = " This type combines a [`SignatureScheme`] and a signature payload produced with that scheme."] # [derive (Debug , Clone)] pub struct DigitallySignedStruct { # [doc = " The [`SignatureScheme`] used to produce the signature."] pub scheme : SignatureScheme , sig : PayloadU16 , }
};
}
