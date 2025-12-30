// Generated macro for DropFlagState (enum)
macro_rules! Depcrate_drop_flag_effectsDropFlagState {
() => {
// Module: crate::drop_flag_effects
// Provides: {"DropFlagState"}
// Dependencies: {}
# [doc = " The value of an inserted drop flag."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum DropFlagState { # [doc = " The tracked value is initialized and needs to be dropped when leaving its scope."] Present , # [doc = " The tracked value is uninitialized or was moved out of and does not need to be dropped when"] # [doc = " leaving its scope."] Absent , }
};
}
