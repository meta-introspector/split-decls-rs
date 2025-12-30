// Generated macro for impl_329 (impl)
macro_rules! Depcrate_load_completionimpl_329 {
() => {
// Module: crate::load::completion
// Provides: {"impl_329"}
// Dependencies: {}
impl < F , C , H > TrackCompletionFuture < F , C , H > { # [doc = " Wraps a future, propagating the tracker into its value if successful."] pub const fn new (completion : C , handle : H , future : F) -> Self { TrackCompletionFuture { future , completion , handle : Some (handle) , } } }
};
}
