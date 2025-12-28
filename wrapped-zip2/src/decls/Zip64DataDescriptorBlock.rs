macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! Zip64DataDescriptorBlock {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct Zip64DataDescriptorBlock { magic : spec :: Magic , pub crc32 : u32 , pub compressed_size : u64 , pub uncompressed_size : u64 , }
    };
}

Zip64DataDescriptorBlock!()