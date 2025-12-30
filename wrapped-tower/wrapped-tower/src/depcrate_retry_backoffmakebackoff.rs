// Generated macro for MakeBackoff (trait)
macro_rules! Depcrate_retry_backoffMakeBackoff {
() => {
// Module: crate::retry::backoff
// Provides: {"MakeBackoff"}
// Dependencies: {}
# [doc = " Trait used to construct [`Backoff`] trait implementors."] pub trait MakeBackoff { # [doc = " The backoff type produced by this maker."] type Backoff : Backoff ; # [doc = " Constructs a new backoff type."] fn make_backoff (& mut self) -> Self :: Backoff ; }
};
}
