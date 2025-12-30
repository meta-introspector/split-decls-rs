// Generated macro for DateLengths (struct)
macro_rules! Depcrate_datetime_legacyDateLengths {
() => {
// Module: crate::datetime::legacy
// Provides: {"DateLengths"}
// Dependencies: {}
# [doc = " Pattern data for dates."] pub struct DateLengths < 'data > { # [doc = " Date pattern data, broken down by pattern length."] pub date : LengthPatterns < 'data > , # [doc = " Patterns used to combine date and time length patterns into full date_time patterns."] pub length_combinations : icu :: datetime :: provider :: skeleton :: GenericLengthPatterns < 'data > , }
};
}
