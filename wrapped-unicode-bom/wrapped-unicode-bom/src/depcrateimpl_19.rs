// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl AsRef < str > for Bom { # [doc = " Returns a `&str` representation of the BOM type."] fn as_ref (& self) -> & str { match * self { Bom :: Null => "[not set]" , Bom :: Bocu1 => "BOCU-1" , Bom :: Gb18030 => "GB 18030" , Bom :: Scsu => "SCSU" , Bom :: UtfEbcdic => "UTF-EBCDIC" , Bom :: Utf1 => "UTF-1" , Bom :: Utf7 => "UTF-7" , Bom :: Utf8 => "UTF-8" , Bom :: Utf16Be => "UTF-16 (big-endian)" , Bom :: Utf16Le => "UTF-16 (little-endian)" , Bom :: Utf32Be => "UTF-32 (big-endian)" , Bom :: Utf32Le => "UTF-32 (little-endian)" , } } }
};
}
