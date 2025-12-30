// Generated macro for Picker (trait)
macro_rules! Depcrate_steerPicker {
() => {
// Module: crate::steer
// Provides: {"Picker"}
// Dependencies: {}
# [doc = " This is how callers of [`Steer`] tell it which `Service` a `Req` corresponds to."] pub trait Picker < S , Req > { # [doc = " Return an index into the iterator of `Service` passed to [`Steer::new`]."] fn pick (& mut self , r : & Req , services : & [S]) -> usize ; }
};
}
