// Generated macro for Timeout (trait)
macro_rules! Depcrate_ansiTimeout {
() => {
// Module: crate::ansi
// Provides: {"Timeout"}
// Dependencies: {}
# [doc = " Interface for creating timeouts and checking their expiry."] # [doc = ""] # [doc = " This is internally used by the [`Processor`] to handle synchronized"] # [doc = " updates."] pub trait Timeout : Default { # [doc = " Sets the timeout for the next synchronized update."] # [doc = ""] # [doc = " The `duration` parameter specifies the duration of the timeout. Once the"] # [doc = " specified duration has elapsed, the synchronized update rotuine can be"] # [doc = " performed."] fn set_timeout (& mut self , duration : Duration) ; # [doc = " Clear the current timeout."] fn clear_timeout (& mut self) ; # [doc = " Returns whether a timeout is currently active and has not yet expired."] fn pending_timeout (& self) -> bool ; }
};
}
