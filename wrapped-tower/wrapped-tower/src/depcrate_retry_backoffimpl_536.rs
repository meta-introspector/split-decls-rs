// Generated macro for impl_536 (impl)
macro_rules! Depcrate_retry_backoffimpl_536 {
() => {
// Module: crate::retry::backoff
// Provides: {"impl_536"}
// Dependencies: {}
impl < R > MakeBackoff for ExponentialBackoffMaker < R > where R : Rng + Clone , { type Backoff = ExponentialBackoff < R > ; fn make_backoff (& mut self) -> Self :: Backoff { ExponentialBackoff { max : self . max , min : self . min , jitter : self . jitter , rng : self . rng . clone () , iterations : 0 , } } }
};
}
