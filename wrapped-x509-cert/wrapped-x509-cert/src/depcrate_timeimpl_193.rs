// Generated macro for impl_193 (impl)
macro_rules! Depcrate_timeimpl_193 {
() => {
// Module: crate::time
// Provides: {"impl_193"}
// Dependencies: {}
impl FromStr for Time { type Err = der :: Error ; fn from_str (input : & str) -> der :: Result < Self > { let datetime = DateTime :: from_str (input) ? ; Ok (Self :: from (datetime)) } }
};
}
