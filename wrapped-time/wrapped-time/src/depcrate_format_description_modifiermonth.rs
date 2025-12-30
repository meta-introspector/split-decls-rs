// Generated macro for Month (struct)
macro_rules! Depcrate_format_description_modifierMonth {
() => {
// Module: crate::format_description::modifier
// Provides: {"Month"}
// Dependencies: {}
# [doc = " Month of the year."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Month { # [doc = " The padding to obtain the minimum width."] pub padding : Padding , # [doc = " What form of representation should be used?"] pub repr : MonthRepr , # [doc = " Is the value case sensitive when parsing?"] pub case_sensitive : bool , }
};
}
