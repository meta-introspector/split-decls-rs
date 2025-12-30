// Generated macro for DerefSource (enum)
macro_rules! Depcrate_shimDerefSource {
() => {
// Module: crate::shim
// Provides: {"DerefSource"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq)] enum DerefSource { # [doc = " `fn shim(&self) { inner(*self )}`."] ImmRef , # [doc = " `fn shim(&mut self) { inner(*self )}`."] MutRef , # [doc = " `fn shim(*mut self) { inner(*self )}`."] MutPtr , }
};
}
