// Generated macro for impl_98 (impl)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerimpl_98 {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"impl_98"}
// Dependencies: {}
impl AudioWorkletHandle { # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::thread()`]."] pub (crate) const fn thread (& self) -> & Thread { & self . thread } # [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletHandle::release()`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`ThreadMemory::release()`]."] pub (crate) unsafe fn release (self) -> Result < () , Self > { let result = unsafe { self . memory . release () } ; match result { Ok (()) => { # [cfg (feature = "message")] super :: main :: DESTROY_SENDER . get () . expect ("sending `ThreadId` before `DESTROY_SENDER` is initialized") . send (self . thread . id ()) . expect ("`Receiver` was somehow dropped from the main thread") ; Ok (()) } Err (memory) => Err (Self { thread : self . thread , memory , }) , } } }
};
}
