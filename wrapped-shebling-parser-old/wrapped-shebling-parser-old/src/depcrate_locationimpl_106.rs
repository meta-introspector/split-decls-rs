// Generated macro for impl_106 (impl)
macro_rules! Depcrate_locationimpl_106 {
() => {
// Module: crate::location
// Provides: {"impl_106"}
// Dependencies: {}
impl Range { # [doc = " Creates a new range from the given start and end locations."] pub (crate) fn new (start : impl Into < Location > , end : impl Into < Location >) -> Self { Self { start : start . into () , end : end . into () , } } # [doc = " Returns the start location of the range."] pub (crate) fn start (& self) -> & Location { & self . start } # [doc = " Returns the end location of the range."] pub (crate) fn end (& self) -> & Location { & self . end } }
};
}
