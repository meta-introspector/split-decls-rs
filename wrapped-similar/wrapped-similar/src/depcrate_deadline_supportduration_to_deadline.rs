// Generated macro for duration_to_deadline (function)
macro_rules! Depcrate_deadline_supportduration_to_deadline {
() => {
// Module: crate::deadline_support
// Provides: {"duration_to_deadline"}
// Dependencies: {}
# [doc = " Converst a duration into a deadline.  This can be a noop on wasm"] # [allow (unused)] pub fn duration_to_deadline (add : Duration) -> Option < Instant > { # [allow (unreachable_code)] # [cfg (all (target_arch = "wasm32" , not (feature = "wasm32_web_time")))] { return None ; } Instant :: now () . checked_add (add) }
};
}
