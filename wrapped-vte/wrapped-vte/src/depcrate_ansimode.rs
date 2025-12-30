// Generated macro for Mode (enum)
macro_rules! Depcrate_ansiMode {
() => {
// Module: crate::ansi
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " Wrapper for the ANSI modes."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Mode { # [doc = " Known ANSI mode."] Named (NamedMode) , # [doc = " Unidentified publc mode."] Unknown (u16) , }
};
}
