// Generated macro for Padding (enum)
macro_rules! Depcrate_format_description_modifierPadding {
() => {
// Module: crate::format_description::modifier
// Provides: {"Padding"}
// Dependencies: {}
# [doc = " Type of padding to ensure a minimum width."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Padding { # [doc = " A space character (` `) should be used as padding."] Space , # [doc = " A zero character (`0`) should be used as padding."] Zero , # [doc = " There is no padding. This can result in a width below the otherwise minimum number of"] # [doc = " characters."] None , }
};
}
