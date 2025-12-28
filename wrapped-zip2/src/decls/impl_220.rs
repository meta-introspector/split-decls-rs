macro_rules! deps {
    () => {
        Magic!();
        ZipDataDescriptorBlock!();
        FixedSizeBlock!();
        ZipError!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl FixedSizeBlock for ZipDataDescriptorBlock { const MAGIC : spec :: Magic = spec :: Magic :: DATA_DESCRIPTOR_SIGNATURE ; # [inline (always)] fn magic (self) -> spec :: Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid data descriptor header") ; to_and_from_le ! [(magic , spec :: Magic) , (crc32 , u32) , (compressed_size , u32) , (uncompressed_size , u32) ,] ; }
    };
}

impl_220!()