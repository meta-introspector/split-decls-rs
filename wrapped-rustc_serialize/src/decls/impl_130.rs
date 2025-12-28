macro_rules! deps {
    () => {
        FileEncoder!();
        IntEncodedWithFixedSize!();
        Encodable!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl Encodable < FileEncoder > for IntEncodedWithFixedSize { # [inline] fn encode (& self , e : & mut FileEncoder) { let start_pos = e . position () ; e . write_array (self . 0 . to_le_bytes ()) ; let end_pos = e . position () ; debug_assert_eq ! ((end_pos - start_pos) , IntEncodedWithFixedSize :: ENCODED_SIZE) ; } }
    };
}

impl_130!();