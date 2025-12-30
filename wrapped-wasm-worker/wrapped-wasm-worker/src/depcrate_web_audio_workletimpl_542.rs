// Generated macro for impl_542 (impl)
macro_rules! Depcrate_web_audio_workletimpl_542 {
() => {
// Module: crate::web::audio_worklet
// Provides: {"impl_542"}
// Dependencies: {}
impl Future for RegisterThreadFuture { type Output = io :: Result < AudioWorkletHandle > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) . map_ok (AudioWorkletHandle) } }
};
}
