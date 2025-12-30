// Generated macro for Zip64DataDescriptorBlock (struct)
macro_rules! Depcrate_typesZip64DataDescriptorBlock {
() => {
// Module: crate::types
// Provides: {"Zip64DataDescriptorBlock"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct Zip64DataDescriptorBlock { magic : spec :: Magic , pub crc32 : u32 , pub compressed_size : u64 , pub uncompressed_size : u64 , }
};
}
