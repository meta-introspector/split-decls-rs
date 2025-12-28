macro_rules! deps {
    () => {
        ExtraFieldMagic!();
    };
}

macro_rules! Zip64ExtraFieldBlock {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub (crate) struct Zip64ExtraFieldBlock { magic : spec :: ExtraFieldMagic , size : u16 , uncompressed_size : Option < u64 > , compressed_size : Option < u64 > , header_start : Option < u64 > , }
    };
}

Zip64ExtraFieldBlock!();