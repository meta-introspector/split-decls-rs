// Generated macro for Year (struct)
macro_rules! Depcrate_format_description_modifierYear {
() => {
// Module: crate::format_description::modifier
// Provides: {"Year"}
// Dependencies: {}
# [doc = " Year of the date."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Year { # [doc = " The padding to obtain the minimum width."] pub padding : Padding , # [doc = " What kind of representation should be used?"] pub repr : YearRepr , # [doc = " What range of years is supported?"] pub range : YearRange , # [doc = " Whether the value is based on the ISO week number or the Gregorian calendar."] pub iso_week_based : bool , # [doc = " Whether the `+` sign is present when a positive year contains fewer than five digits."] pub sign_is_mandatory : bool , }
};
}
