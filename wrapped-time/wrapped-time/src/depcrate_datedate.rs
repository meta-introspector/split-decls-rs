// Generated macro for Date (struct)
macro_rules! Depcrate_dateDate {
() => {
// Module: crate::date
// Provides: {"Date"}
// Dependencies: {}
# [doc = " Date in the proleptic Gregorian calendar."] # [doc = ""] # [doc = " By default, years between ±9999 inclusive are representable. This can be expanded to ±999,999"] # [doc = " inclusive by enabling the `large-dates` crate feature. Doing so has performance implications"] # [doc = " and introduces some ambiguities when parsing."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct Date { # [doc = " Bitpacked field containing the year, ordinal, and whether the year is a leap year."] value : NonZero < i32 > , }
};
}
