// Generated macro for deadline_exceeded (function)
macro_rules! Depcrate_deadline_supportdeadline_exceeded {
() => {
// Module: crate::deadline_support
// Provides: {"deadline_exceeded"}
// Dependencies: {}
# [doc = " Checks if a deadline was exeeded."] pub fn deadline_exceeded (deadline : Option < Instant >) -> bool { # [allow (unreachable_code)] match deadline { Some (deadline) => { # [cfg (all (target_arch = "wasm32" , not (feature = "wasm32_web_time")))] { return false ; } Instant :: now () > deadline } None => false , } }
};
}
