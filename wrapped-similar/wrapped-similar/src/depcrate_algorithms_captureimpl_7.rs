// Generated macro for impl_7 (impl)
macro_rules! Depcrate_algorithms_captureimpl_7 {
() => {
// Module: crate::algorithms::capture
// Provides: {"impl_7"}
// Dependencies: {}
impl Capture { # [doc = " Creates a new capture hook."] pub fn new () -> Capture { Capture :: default () } # [doc = " Converts the capture hook into a vector of ops."] pub fn into_ops (self) -> Vec < DiffOp > { self . 0 } # [doc = " Isolate change clusters by eliminating ranges with no changes."] # [doc = ""] # [doc = " This is equivalent to calling [`group_diff_ops`] on [`Capture::into_ops`]."] pub fn into_grouped_ops (self , n : usize) -> Vec < Vec < DiffOp > > { group_diff_ops (self . into_ops () , n) } # [doc = " Accesses the captured operations."] pub fn ops (& self) -> & [DiffOp] { & self . 0 } }
};
}
