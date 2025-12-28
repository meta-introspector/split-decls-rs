macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < & [u8] > for Bom { # [doc = " Detect the BOM type from a byte array."] fn from (slice : & [u8]) -> Self { if slice . len () >= 2 { match slice [0] { 0 => { if compare_tail ! (slice , [0 , 0xfe , 0xff]) { return Bom :: Utf32Be ; } } 0x0e => { if compare_tail ! (slice , [0xfe , 0xff]) { return Bom :: Scsu ; } } 0x2b => { if compare_tail ! (slice , 4 , [0x2f , 0x76] , 1) && (slice [3] == 0x38 || slice [3] == 0x39 || slice [3] == 0x2b || slice [3] == 0x2f) { return Bom :: Utf7 ; } } 0x84 => { if compare_tail ! (slice , [0x31 , 0x95 , 0x33]) { return Bom :: Gb18030 ; } } 0xdd => { if compare_tail ! (slice , [0x73 , 0x66 , 0x73]) { return Bom :: UtfEbcdic ; } } 0xef => { if compare_tail ! (slice , [0xbb , 0xbf]) { return Bom :: Utf8 ; } } 0xf7 => { if compare_tail ! (slice , [0x64 , 0x4c]) { return Bom :: Utf1 ; } } 0xfb => { if compare_tail ! (slice , [0xee , 0x28]) { return Bom :: Bocu1 ; } } 0xfe => { if slice [1] == 0xff { return Bom :: Utf16Be ; } } 0xff => { if slice [1] == 0xfe { if compare_tail ! (slice , [0 , 0] , 2) { return Bom :: Utf32Le ; } return Bom :: Utf16Le ; } } _ => { } } } Bom :: Null } }
    };
}

impl_9!()