macro_rules! deps {
    () => {
        UnicodeExtraField!();
        ZipResult!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl UnicodeExtraField { # [doc = " Verifies the checksum and returns the content."] pub fn unwrap_valid (self , ascii_field : & [u8]) -> ZipResult < Box < [u8] > > { let mut crc32 = crc32fast :: Hasher :: new () ; crc32 . update (ascii_field) ; let actual_crc32 = crc32 . finalize () ; if self . crc32 != actual_crc32 { return Err (invalid ! ("CRC32 checksum failed on Unicode extra field")) ; } Ok (self . content) } }
    };
}

impl_69!();