// Generated macro for impl_52 (impl)
macro_rules! Depcrate_parametersimpl_52 {
() => {
// Module: crate::parameters
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for EcParameters { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (decoder : & mut R , header : Header) -> der :: Result < Self > { ObjectIdentifier :: decode_value (decoder , header) . map (Self :: NamedCurve) } }
};
}
