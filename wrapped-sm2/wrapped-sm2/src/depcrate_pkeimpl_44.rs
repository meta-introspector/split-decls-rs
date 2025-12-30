// Generated macro for impl_44 (impl)
macro_rules! Depcrate_pkeimpl_44 {
() => {
// Module: crate::pke
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Cipher < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (decoder : & mut R , header : der :: Header ,) -> core :: result :: Result < Self , Self :: Error > { decoder . read_nested (header . length () , | nr | { let x = UintRef :: decode (nr) ? . as_bytes () ; let y = UintRef :: decode (nr) ? . as_bytes () ; let digest = < & 'a OctetStringRef > :: decode (nr) ? . into () ; let cipher = < & 'a OctetStringRef > :: decode (nr) ? . into () ; Ok (Cipher { x : Uint :: from_be_bytes (zero_pad_byte_slice (x) ?) , y : Uint :: from_be_bytes (zero_pad_byte_slice (y) ?) , digest , cipher , }) }) } }
};
}
