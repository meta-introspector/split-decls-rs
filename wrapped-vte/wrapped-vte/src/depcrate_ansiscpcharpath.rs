// Generated macro for ScpCharPath (enum)
macro_rules! Depcrate_ansiScpCharPath {
() => {
// Module: crate::ansi
// Provides: {"ScpCharPath"}
// Dependencies: {}
# [doc = " SCP control's first parameter which determines character path."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ScpCharPath { # [doc = " SCP's first parameter value of 0. Behavior is implementation defined."] Default , # [doc = " SCP's first parameter value of 1 which sets character path to"] # [doc = " LEFT-TO-RIGHT."] LTR , # [doc = " SCP's first parameter value of 2 which sets character path to"] # [doc = " RIGHT-TO-LEFT."] RTL , }
};
}
