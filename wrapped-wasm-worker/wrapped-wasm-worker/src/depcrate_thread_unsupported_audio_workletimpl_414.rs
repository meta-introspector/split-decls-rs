// Generated macro for impl_414 (impl)
macro_rules! Depcrate_thread_unsupported_audio_workletimpl_414 {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"impl_414"}
// Dependencies: {}
impl AudioWorkletHandle { # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::thread()`]."] # [allow (clippy :: unused_self)] pub (crate) const fn thread (& self) -> & Thread { unreachable ! () } # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::release()`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is only marked `unsafe` for compatibility with the atomics"] # [doc = " implementation."] # [allow (clippy :: unused_self)] pub (crate) unsafe fn release (self) -> Result < () , Self > { unreachable ! ("reached `register_thread()` without atomics target feature") } }
};
}
