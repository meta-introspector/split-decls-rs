// Generated macro for MAX_YEAR (const)
macro_rules! Depcrate_dateMAX_YEAR {
() => {
// Module: crate::date
// Provides: {"MAX_YEAR"}
// Dependencies: {}
# [doc = " The maximum valid year."] pub (crate) const MAX_YEAR : i32 = if cfg ! (feature = "large-dates") { 999_999 } else { 9999 } ;
};
}
