// Generated macro for SalsaAsRef (trait)
macro_rules! Depcrate_return_modeSalsaAsRef {
() => {
// Module: crate::return_mode
// Provides: {"SalsaAsRef"}
// Dependencies: {}
# [doc = " Used to determine the return type and value for tracked fields and functions annotated with `returns(as_ref)`."] pub trait SalsaAsRef { type AsRef < 'a > where Self : 'a ; fn as_ref (& self) -> Self :: AsRef < '_ > ; }
};
}
