// Generated macro for Backoff (trait)
macro_rules! Depcrate_retry_backoffBackoff {
() => {
// Module: crate::retry::backoff
// Provides: {"Backoff"}
// Dependencies: {}
# [doc = " A backoff trait where a single mutable reference represents a single"] # [doc = " backoff session. Implementors must also implement [`Clone`] which will"] # [doc = " reset the backoff back to the default state for the next session."] pub trait Backoff { # [doc = " The future associated with each backoff. This usually will be some sort"] # [doc = " of timer."] type Future : Future < Output = () > ; # [doc = " Initiate the next backoff in the sequence."] fn next_backoff (& mut self) -> Self :: Future ; }
};
}
