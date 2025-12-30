// Generated macro for YearRepr (enum)
macro_rules! Depcrate_format_description_modifierYearRepr {
() => {
// Module: crate::format_description::modifier
// Provides: {"YearRepr"}
// Dependencies: {}
# [doc = " The representation used for a year value."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum YearRepr { # [doc = " The full value of the year."] Full , # [doc = " All digits except the last two. Includes the sign, if any."] Century , # [doc = " Only the last two digits of the year."] LastTwo , }
};
}
