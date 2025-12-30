// Generated macro for Oneshot (trait)
macro_rules! Depcrate_oneshot_traitsOneshot {
() => {
// Module: crate::oneshot::traits
// Provides: {"Oneshot"}
// Dependencies: {}
# [doc = " A future-based worker that for each input, one output is produced."] pub trait Oneshot : Future { # [doc = " Incoming message type."] type Input ; # [doc = " Creates an oneshot worker."] fn create (input : Self :: Input) -> Self ; }
};
}
