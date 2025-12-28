macro_rules! deps {
    () => {
        ExtraFieldMagic!();
        Zip64ExtraFieldBlock!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl Zip64ExtraFieldBlock { pub (crate) fn maybe_new (large_file : bool , uncompressed_size : u64 , compressed_size : u64 , header_start : u64 ,) -> Option < Zip64ExtraFieldBlock > { let mut size : u16 = 0 ; let uncompressed_size = if uncompressed_size >= ZIP64_BYTES_THR || large_file { size += mem :: size_of :: < u64 > () as u16 ; Some (uncompressed_size) } else { None } ; let compressed_size = if compressed_size >= ZIP64_BYTES_THR || large_file { size += mem :: size_of :: < u64 > () as u16 ; Some (compressed_size) } else { None } ; let header_start = if header_start >= ZIP64_BYTES_THR { size += mem :: size_of :: < u64 > () as u16 ; Some (header_start) } else { None } ; if size == 0 { return None ; } Some (Zip64ExtraFieldBlock { magic : spec :: ExtraFieldMagic :: ZIP64_EXTRA_FIELD_TAG , size , uncompressed_size , compressed_size , header_start , }) } }
    };
}

impl_216!()