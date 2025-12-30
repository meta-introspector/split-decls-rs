// Generated macro for impl_331 (impl)
macro_rules! Depcrate_load_completionimpl_331 {
() => {
// Module: crate::load::completion
// Provides: {"impl_331"}
// Dependencies: {}
impl < H , V > TrackCompletion < H , V > for CompleteOnResponse { type Output = V ; fn track_completion (& self , handle : H , value : V) -> V { drop (handle) ; value } }
};
}
