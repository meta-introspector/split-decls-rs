// Generated macro for KeyboardModesApplyBehavior (enum)
macro_rules! Depcrate_ansiKeyboardModesApplyBehavior {
() => {
// Module: crate::ansi
// Provides: {"KeyboardModesApplyBehavior"}
// Dependencies: {}
# [doc = " Describes how the new [`KeyboardModes`] should be applied."] # [repr (u8)] # [derive (Default , Clone , Copy , PartialEq , Eq)] pub enum KeyboardModesApplyBehavior { # [doc = " Replace the active flags with the new ones."] # [default] Replace , # [doc = " Merge the given flags with currently active ones."] Union , # [doc = " Remove the given flags from the active ones."] Difference , }
};
}
