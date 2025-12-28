macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! ZipDataDescriptorBlock {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct ZipDataDescriptorBlock { magic : spec :: Magic , pub crc32 : u32 , pub compressed_size : u32 , pub uncompressed_size : u32 , }
    };
}

ZipDataDescriptorBlock!()