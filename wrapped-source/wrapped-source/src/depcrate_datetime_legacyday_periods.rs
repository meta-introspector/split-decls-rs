// Generated macro for day_periods (module)
macro_rules! Depcrate_datetime_legacyday_periods {
() => {
// Module: crate::datetime::legacy
// Provides: {"day_periods"}
// Dependencies: {}
# [doc = "Formatting symbols for [`DayPeriod`](crate::provider::fields::FieldSymbol::DayPeriod)."] # [doc = ""] # [doc = "For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol)."] pub mod day_periods { use super :: * ; # [doc = "Locale data for DayPeriod corresponding to the symbols."] pub struct Symbols < 'data > { # [doc = " Day period for AM (before noon)."] pub am : Cow < 'data , str > , # [doc = " Day period for PM (after noon)."] pub pm : Cow < 'data , str > , # [doc = " Day period for noon, in locales that support it."] pub noon : Option < Cow < 'data , str > > , # [doc = " Day period for midnight, in locales that support it."] pub midnight : Option < Cow < 'data , str > > , } }
};
}
