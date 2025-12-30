// Generated macro for MIN_YEAR (const)
macro_rules! Depcrate_dateMIN_YEAR {
() => {
// Module: crate::date
// Provides: {"MIN_YEAR"}
// Dependencies: {}
# [doc = " The minimum valid year."] pub (crate) const MIN_YEAR : i32 = if cfg ! (feature = "large-dates") { - 999_999 } else { - 9999 } ;
};
}
