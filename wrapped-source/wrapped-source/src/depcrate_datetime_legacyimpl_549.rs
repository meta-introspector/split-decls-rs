// Generated macro for impl_549 (impl)
macro_rules! Depcrate_datetime_legacyimpl_549 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_549"}
// Dependencies: {}
impl cldr_serde :: ca :: MonthSymbols { fn get (& self , ctx : & (& 'static [TinyStr4] , & str)) -> months :: Symbols < 'static > { if ctx . 0 . len () == 12 && self . 0 . len () == 12 { let mut arr : [Cow < 'static , str > ; 12] = Default :: default () ; for (k , v) in self . 0 . iter () { let index : usize = k . parse () . expect ("CLDR month indices must parse as numbers!") ; if index == 0 { panic ! ("CLDR month indices cannot be zero") ; } arr [index - 1] = Cow :: Owned (v . into ()) ; } for (i , val) in arr . iter () . enumerate () { if val . is_empty () { panic ! ("Solar calendar does not have data for month {i}") ; } } months :: Symbols :: SolarTwelve (arr) } else { let mut map = BTreeMap :: new () ; for (k , v) in self . 0 . iter () { let code = if k == "7-yeartype-leap" && ctx . 1 == "hebrew" { tinystr ! (4 , "M06L") } else { let index : usize = k . parse () . expect ("CLDR month indices must parse as numbers!") ; if index == 0 { panic ! ("CLDR month indices cannot be zero") ; } * ctx . 0 . get (index - 1) . expect ("Found out of bounds month index for calendar") } ; map . insert (MonthCode (code) , v . as_ref ()) ; } months :: Symbols :: Other (map . into_iter () . collect ()) } } }
};
}
