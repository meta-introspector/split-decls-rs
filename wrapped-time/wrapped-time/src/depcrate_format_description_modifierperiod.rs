// Generated macro for Period (struct)
macro_rules! Depcrate_format_description_modifierPeriod {
() => {
// Module: crate::format_description::modifier
// Provides: {"Period"}
// Dependencies: {}
# [doc = " AM/PM part of the time."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Period { # [doc = " Is the period uppercase or lowercase?"] pub is_uppercase : bool , # [doc = " Is the value case sensitive when parsing?"] # [doc = ""] # [doc = " Note that when `false`, the `is_uppercase` field has no effect on parsing behavior."] pub case_sensitive : bool , }
};
}
