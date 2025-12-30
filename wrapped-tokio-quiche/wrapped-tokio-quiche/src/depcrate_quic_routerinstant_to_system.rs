// Generated macro for instant_to_system (function)
macro_rules! Depcrate_quic_routerinstant_to_system {
() => {
// Module: crate::quic::router
// Provides: {"instant_to_system"}
// Dependencies: {}
# [doc = " Converts an [`Instant`] to a [`SystemTime`], based on the current delta"] # [doc = " between both clocks."] fn instant_to_system (ts : Instant) -> SystemTime { let now = Instant :: now () ; let system_now = SystemTime :: now () ; if let Some (delta) = now . checked_duration_since (ts) { return system_now - delta ; } let delta = ts . checked_duration_since (now) . expect ("now < ts") ; system_now + delta }
};
}
