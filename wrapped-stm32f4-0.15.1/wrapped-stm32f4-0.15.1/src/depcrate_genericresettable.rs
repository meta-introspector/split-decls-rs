// Generated macro for Resettable (trait)
macro_rules! Depcrate_genericResettable {
() => {
// Module: crate::generic
// Provides: {"Resettable"}
// Dependencies: {}
# [doc = " Reset value of the register."] # [doc = ""] # [doc = " This value is the initial value for the `write` method. It can also be directly written to the"] # [doc = " register by using the `reset` method."] pub trait Resettable : RegisterSpec { # [doc = " Reset value of the register."] fn reset_value () -> Self :: Ux ; }
};
}
