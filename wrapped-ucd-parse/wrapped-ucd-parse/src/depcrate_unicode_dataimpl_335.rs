// Generated macro for impl_335 (impl)
macro_rules! Depcrate_unicode_dataimpl_335 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_335"}
// Dependencies: {}
impl std :: str :: FromStr for UnicodeDataNumeric { type Err = Error ; fn from_str (s : & str) -> Result < UnicodeDataNumeric , Error > { if s . is_empty () { return err ! ("expected non-empty string for UnicodeDataNumeric value") ; } if let Some (pos) = s . find ('/') { let (snum , sden) = (& s [.. pos] , & s [pos + 1 ..]) ; let num = match snum . parse () { Ok (num) => num , Err (err) => { return err ! ("invalid integer numerator '{}': {}" , snum , err) ; } } ; let den = match sden . parse () { Ok (den) => den , Err (err) => { return err ! ("invalid integer denominator '{}': {}" , sden , err) ; } } ; Ok (UnicodeDataNumeric :: Rational (num , den)) } else { match s . parse () { Ok (den) => Ok (UnicodeDataNumeric :: Integer (den)) , Err (err) => { return err ! ("invalid integer denominator '{}': {}" , s , err) ; } } } } }
};
}
