// Generated macro for WeekdayRepr (enum)
macro_rules! Depcrate_format_description_modifierWeekdayRepr {
() => {
// Module: crate::format_description::modifier
// Provides: {"WeekdayRepr"}
// Dependencies: {}
# [doc = " The representation used for the day of the week."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum WeekdayRepr { # [doc = " The short form of the weekday (e.g. \"Mon\")."] Short , # [doc = " The long form of the weekday (e.g. \"Monday\")."] Long , # [doc = " A numerical representation using Sunday as the first day of the week."] # [doc = ""] # [doc = " Sunday is either 0 or 1, depending on the other modifier's value."] Sunday , # [doc = " A numerical representation using Monday as the first day of the week."] # [doc = ""] # [doc = " Monday is either 0 or 1, depending on the other modifier's value."] Monday , }
};
}
