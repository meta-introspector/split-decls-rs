// Generated macro for impl_157 (impl)
macro_rules! Depcrate_opaqueimpl_157 {
() => {
// Module: crate::opaque
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a > Decodable < MemDecoder < 'a > > for IntEncodedWithFixedSize { # [inline] fn decode (decoder : & mut MemDecoder < 'a >) -> IntEncodedWithFixedSize { let bytes = decoder . read_array :: < { IntEncodedWithFixedSize :: ENCODED_SIZE } > () ; IntEncodedWithFixedSize (u64 :: from_le_bytes (bytes)) } }
};
}
