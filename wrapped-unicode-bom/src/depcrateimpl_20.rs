// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl AsRef < [u8] > for Bom { # [doc = " Returns the BOM byte-array literal."] # [doc = ""] # [doc = " Note that for UTF-7,"] # [doc = " only the first three bytes of the BOM are returned."] # [doc = " That's because the last two bits of the fourth byte"] # [doc = " belong to the following character,"] # [doc = " so it's impossible to return the fourth byte"] # [doc = " without further context."] # [doc = " Possible values for the missing fourth byte"] # [doc = " are `0x38`, `0x39`, `0x2a` and `0x2b`."] fn as_ref (& self) -> & [u8] { match * self { Bom :: Null => & [] , Bom :: Bocu1 => & [0xfb , 0xee , 0x28] , Bom :: Gb18030 => & [0x84 , 0x31 , 0x95 , 0x33] , Bom :: Scsu => & [0x0e , 0xfe , 0xff] , Bom :: UtfEbcdic => & [0xdd , 0x73 , 0x66 , 0x73] , Bom :: Utf1 => & [0xf7 , 0x64 , 0x4c] , Bom :: Utf7 => & [0x2b , 0x2f , 0x76] , Bom :: Utf8 => & [0xef , 0xbb , 0xbf] , Bom :: Utf16Be => & [0xfe , 0xff] , Bom :: Utf16Le => & [0xff , 0xfe] , Bom :: Utf32Be => & [0 , 0 , 0xfe , 0xff] , Bom :: Utf32Le => & [0xff , 0xfe , 0 , 0] , } } }
};
}
