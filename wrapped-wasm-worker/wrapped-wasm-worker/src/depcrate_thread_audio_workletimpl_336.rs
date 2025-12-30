// Generated macro for impl_336 (impl)
macro_rules! Depcrate_thread_audio_workletimpl_336 {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"impl_336"}
// Dependencies: {}
impl AudioWorkletHandle { # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::thread()`]."] pub (crate) const fn thread (& self) -> & Thread { self . 0 . thread () } # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::release()`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`AudioWorkletHandle::release()`](audio_worklet::AudioWorkletHandle::release)."] pub (crate) unsafe fn release (self) -> Result < () , Self > { unsafe { self . 0 . release () } . map_err (Self) } }
};
}
