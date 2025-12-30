// Generated macro for impl_535 (impl)
macro_rules! Depcrate_datetime_legacyimpl_535 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_535"}
// Dependencies: {}
impl months :: Symbols < '_ > { # [doc = " Get the symbol for the given month code"] pub fn get (& self , code : MonthCode) -> Option < & str > { use tinystr :: { tinystr , TinyStr4 } ; match * self { Self :: SolarTwelve (ref arr) => { const CODE_1 : TinyStr4 = tinystr ! (4 , "M01") ; const CODE_2 : TinyStr4 = tinystr ! (4 , "M02") ; const CODE_3 : TinyStr4 = tinystr ! (4 , "M03") ; const CODE_4 : TinyStr4 = tinystr ! (4 , "M04") ; const CODE_5 : TinyStr4 = tinystr ! (4 , "M05") ; const CODE_6 : TinyStr4 = tinystr ! (4 , "M06") ; const CODE_7 : TinyStr4 = tinystr ! (4 , "M07") ; const CODE_8 : TinyStr4 = tinystr ! (4 , "M08") ; const CODE_9 : TinyStr4 = tinystr ! (4 , "M09") ; const CODE_10 : TinyStr4 = tinystr ! (4 , "M10") ; const CODE_11 : TinyStr4 = tinystr ! (4 , "M11") ; const CODE_12 : TinyStr4 = tinystr ! (4 , "M12") ; let idx = match code . 0 { CODE_1 => 0 , CODE_2 => 1 , CODE_3 => 2 , CODE_4 => 3 , CODE_5 => 4 , CODE_6 => 5 , CODE_7 => 6 , CODE_8 => 7 , CODE_9 => 8 , CODE_10 => 9 , CODE_11 => 10 , CODE_12 => 11 , _ => return None , } ; arr . get (idx) . map (| x | & * * x) } Self :: Other (ref map) => map . get (& code) , } } }
};
}
