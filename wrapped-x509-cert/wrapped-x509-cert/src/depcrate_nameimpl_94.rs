// Generated macro for impl_94 (impl)
macro_rules! Depcrate_nameimpl_94 {
() => {
// Module: crate::name
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Name { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (decoder : & mut R , header : Header) -> der :: Result < Self > { Ok (Self (RdnSequence :: decode_value (decoder , header) ?)) } }
};
}
