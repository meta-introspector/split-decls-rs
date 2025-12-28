macro_rules! deps {
    () => {
        MemDecoder!();
        IntEncodedWithFixedSize!();
        Decodable!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a > Decodable < MemDecoder < 'a > > for IntEncodedWithFixedSize { # [inline] fn decode (decoder : & mut MemDecoder < 'a >) -> IntEncodedWithFixedSize { let bytes = decoder . read_array :: < { IntEncodedWithFixedSize :: ENCODED_SIZE } > () ; IntEncodedWithFixedSize (u64 :: from_le_bytes (bytes)) } }
    };
}

impl_131!()