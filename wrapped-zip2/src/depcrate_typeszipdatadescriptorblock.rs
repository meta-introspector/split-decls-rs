// Generated macro for ZipDataDescriptorBlock (struct)
macro_rules! Depcrate_typesZipDataDescriptorBlock {
() => {
// Module: crate::types
// Provides: {"ZipDataDescriptorBlock"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct ZipDataDescriptorBlock { magic : spec :: Magic , pub crc32 : u32 , pub compressed_size : u32 , pub uncompressed_size : u32 , }
};
}
