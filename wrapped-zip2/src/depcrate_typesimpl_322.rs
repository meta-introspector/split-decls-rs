// Generated macro for impl_322 (impl)
macro_rules! Depcrate_typesimpl_322 {
() => {
// Module: crate::types
// Provides: {"impl_322"}
// Dependencies: {}
impl Zip64ExtraFieldBlock { pub fn full_size (& self) -> usize { assert ! (self . size > 0) ; self . size as usize + mem :: size_of :: < spec :: ExtraFieldMagic > () + mem :: size_of :: < u16 > () } pub fn serialize (self) -> Box < [u8] > { let Self { magic , size , uncompressed_size , compressed_size , header_start , } = self ; let full_size = self . full_size () ; let mut ret = Vec :: with_capacity (full_size) ; ret . extend (magic . to_le_bytes ()) ; ret . extend (u16 :: to_le_bytes (size)) ; if let Some (uncompressed_size) = uncompressed_size { ret . extend (u64 :: to_le_bytes (uncompressed_size)) ; } if let Some (compressed_size) = compressed_size { ret . extend (u64 :: to_le_bytes (compressed_size)) ; } if let Some (header_start) = header_start { ret . extend (u64 :: to_le_bytes (header_start)) ; } debug_assert_eq ! (ret . len () , full_size) ; ret . into_boxed_slice () } }
};
}
