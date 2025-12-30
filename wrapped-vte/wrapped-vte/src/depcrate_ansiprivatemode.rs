// Generated macro for PrivateMode (enum)
macro_rules! Depcrate_ansiPrivateMode {
() => {
// Module: crate::ansi
// Provides: {"PrivateMode"}
// Dependencies: {}
# [doc = " Wrapper for the private DEC modes."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum PrivateMode { # [doc = " Known private mode."] Named (NamedPrivateMode) , # [doc = " Unknown private mode."] Unknown (u16) , }
};
}
