// Generated macro for impl_542 (impl)
macro_rules! Depcrate_datetime_legacyimpl_542 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_542"}
// Dependencies: {}
impl < 'a > From < & months :: Symbols < 'a > > for MonthNames < 'a > { fn from (other : & months :: Symbols < 'a >) -> Self { match other { months :: Symbols :: SolarTwelve (cow_list) => { let vec : alloc :: vec :: Vec < & str > = cow_list . iter () . map (| x | & * * x) . collect () ; MonthNames :: Linear ((& vec) . into ()) } months :: Symbols :: Other (zero_map) => { let mut vec = vec ! ["" ; 24] ; # [allow (deprecated)] for (k , v) in zero_map . iter () { let Some ((number , leap)) = MonthCode (* k) . parsed () else { debug_assert ! (false , "Found unknown month code {k}") ; continue ; } ; let offset = if leap { 12 } else { 0 } ; if let Some (entry) = vec . get_mut ((number + offset - 1) as usize) { * entry = v ; } else { debug_assert ! (false , "Found out of bounds hebrew month code {k}") } } MonthNames :: LeapLinear ((& vec) . into ()) } } } }
};
}
