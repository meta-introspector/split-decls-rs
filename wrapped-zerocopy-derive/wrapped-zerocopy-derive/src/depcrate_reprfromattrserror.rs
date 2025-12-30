// Generated macro for FromAttrsError (enum)
macro_rules! Depcrate_reprFromAttrsError {
() => {
// Module: crate::repr
// Provides: {"FromAttrsError"}
// Dependencies: {}
# [doc = " The error returned from [`Repr::from_attrs`]."] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] enum FromAttrsError { FromRawReprs (FromRawReprsError < UnsupportedReprError >) , Unrecognized , }
};
}
