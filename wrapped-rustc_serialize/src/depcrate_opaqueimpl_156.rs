// Generated macro for impl_156 (impl)
macro_rules! Depcrate_opaqueimpl_156 {
() => {
// Module: crate::opaque
// Provides: {"impl_156"}
// Dependencies: {}
impl Encodable < FileEncoder > for IntEncodedWithFixedSize { # [inline] fn encode (& self , e : & mut FileEncoder) { let start_pos = e . position () ; e . write_array (self . 0 . to_le_bytes ()) ; let end_pos = e . position () ; debug_assert_eq ! ((end_pos - start_pos) , IntEncodedWithFixedSize :: ENCODED_SIZE) ; } }
};
}
