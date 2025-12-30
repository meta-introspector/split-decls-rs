// Generated macro for impl_334 (impl)
macro_rules! Depcrate_thread_audio_workletimpl_334 {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"impl_334"}
// Dependencies: {}
impl Future for RegisterThreadFuture { type Output = io :: Result < AudioWorkletHandle > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) . map_ok (AudioWorkletHandle) } }
};
}
