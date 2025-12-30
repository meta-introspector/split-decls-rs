// Generated macro for as_ref_arr (function)
macro_rules! Depcrate_testas_ref_arr {
() => {
// Module: crate::test
// Provides: {"as_ref_arr"}
// Dependencies: {}
# [test] fn as_ref_arr () { assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Null) , & []) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Bocu1) , & [0xfb , 0xee , 0x28]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Gb18030) , & [0x84 , 0x31 , 0x95 , 0x33]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Scsu) , & [0x0e , 0xfe , 0xff]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: UtfEbcdic) , & [0xdd , 0x73 , 0x66 , 0x73]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf1) , & [0xf7 , 0x64 , 0x4c]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf7) , & [0x2b , 0x2f , 0x76]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf8) , & [0xef , 0xbb , 0xbf]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf16Be) , & [0xfe , 0xff]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf16Le) , & [0xff , 0xfe]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf32Be) , & [0 , 0 , 0xfe , 0xff]) ; assert_eq ! (AsRef ::< [u8] >:: as_ref (& Bom :: Utf32Le) , & [0xff , 0xfe , 0 , 0]) ; }
};
}
