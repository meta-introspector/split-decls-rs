// Generated macro for impl_328 (impl)
macro_rules! Depcrate_typesimpl_328 {
() => {
// Module: crate::types
// Provides: {"impl_328"}
// Dependencies: {}
impl FixedSizeBlock for Zip64DataDescriptorBlock { const MAGIC : spec :: Magic = spec :: Magic :: DATA_DESCRIPTOR_SIGNATURE ; # [inline (always)] fn magic (self) -> spec :: Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid zip64 data descriptor header") ; to_and_from_le ! [(magic , spec :: Magic) , (crc32 , u32) , (compressed_size , u64) , (uncompressed_size , u64) ,] ; }
};
}
