// Generated macro for DateSymbols (struct)
macro_rules! Depcrate_datetime_legacyDateSymbols {
() => {
// Module: crate::datetime::legacy
// Provides: {"DateSymbols"}
// Dependencies: {}
# [doc = " Symbol data for the months, weekdays, and eras needed to format a date."] # [doc = ""] # [doc = " For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol)."] pub struct DateSymbols < 'data > { # [doc = " Symbol data for months."] pub months : months :: Contexts < 'data > , # [doc = " Symbol data for weekdays."] pub weekdays : weekdays :: Contexts < 'data > , }
};
}
