// Generated macro for impl_94 (impl)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerimpl_94 {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"impl_94"}
// Dependencies: {}
impl Drop for RegisterThreadFuture { fn drop (& mut self) { let Some (state) = self . 0 . take () else { return } ; if ! matches ! (state , State :: Error (_)) { wasm_bindgen_futures :: spawn_local (async move { let _ = Self (Some (state)) . await ; }) ; } } }
};
}
