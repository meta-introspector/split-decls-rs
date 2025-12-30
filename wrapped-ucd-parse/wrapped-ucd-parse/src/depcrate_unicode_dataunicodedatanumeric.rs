// Generated macro for UnicodeDataNumeric (enum)
macro_rules! Depcrate_unicode_dataUnicodeDataNumeric {
() => {
// Module: crate::unicode_data
// Provides: {"UnicodeDataNumeric"}
// Dependencies: {}
# [doc = " A numeric value corresponding to characters with `Numeric_Type=Numeric`."] # [doc = ""] # [doc = " A numeric value can either be a signed integer or a rational number."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum UnicodeDataNumeric { # [doc = " An integer."] Integer (i64) , # [doc = " A rational number. The first is the numerator and the latter is the"] # [doc = " denominator."] Rational (i64 , i64) , }
};
}
