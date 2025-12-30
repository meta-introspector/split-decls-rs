// Generated macro for Filter (enum)
macro_rules! DepcrateFilter {
() => {
// Module: crate
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " A filter to control which suggestion should be applied."] # [derive (Debug , Clone , Copy)] pub enum Filter { # [doc = " For [`diagnostics::Applicability::MachineApplicable`] only."] MachineApplicableOnly , # [doc = " Everything is included. YOLO!"] Everything , }
};
}
