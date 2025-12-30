// Generated macro for impl_411 (impl)
macro_rules! Depcrate_thread_unsupported_audio_workletimpl_411 {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"impl_411"}
// Dependencies: {}
impl Future for RegisterThreadFuture { type Output = io :: Result < AudioWorkletHandle > ; fn poll (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (Err (self . error . take () . expect ("polled after completion"))) } }
};
}
