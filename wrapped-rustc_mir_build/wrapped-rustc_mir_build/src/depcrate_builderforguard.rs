// Generated macro for ForGuard (enum)
macro_rules! Depcrate_builderForGuard {
() => {
// Module: crate::builder
// Provides: {"ForGuard"}
// Dependencies: {}
# [doc = " `ForGuard` indicates whether we are talking about:"] # [doc = "   1. The variable for use outside of guard expressions, or"] # [doc = "   2. The temp that holds reference to (1.), which is actually what the"] # [doc = "      guard expressions see."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum ForGuard { RefWithinGuard , OutsideGuard , }
};
}
