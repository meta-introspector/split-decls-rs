// Generated macro for local_offset (module)
macro_rules! Depcrate_utillocal_offset {
() => {
// Module: crate::util
// Provides: {"local_offset"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "local-offset")] # [expect (clippy :: missing_const_for_fn , reason = "no longer used; original implementation was not const")] # [deprecated (since = "0.3.37" , note = "no longer needed; TZ is refreshed manually")] pub mod local_offset { # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Soundness { Sound , Unsound , } # [inline] pub unsafe fn set_soundness (_ : Soundness) { } # [inline] pub fn get_soundness () -> Soundness { Soundness :: Sound } }
};
}
