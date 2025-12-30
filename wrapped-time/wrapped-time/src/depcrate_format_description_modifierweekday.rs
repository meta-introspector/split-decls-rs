// Generated macro for Weekday (struct)
macro_rules! Depcrate_format_description_modifierWeekday {
() => {
// Module: crate::format_description::modifier
// Provides: {"Weekday"}
// Dependencies: {}
# [doc = " Day of the week."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Weekday { # [doc = " What form of representation should be used?"] pub repr : WeekdayRepr , # [doc = " When using a numerical representation, should it be zero or one-indexed?"] pub one_indexed : bool , # [doc = " Is the value case sensitive when parsing?"] pub case_sensitive : bool , }
};
}
