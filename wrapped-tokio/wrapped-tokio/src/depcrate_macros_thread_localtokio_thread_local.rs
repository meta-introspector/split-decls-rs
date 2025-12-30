// Generated macro for tokio_thread_local (macro)
macro_rules! Depcrate_macros_thread_localtokio_thread_local {
() => {
// Module: crate::macros::thread_local
// Provides: {"tokio_thread_local"}
// Dependencies: {}
# [cfg (not (all (loom , test)))] macro_rules ! tokio_thread_local { ($ ($ tts : tt) +) => { :: std :: thread_local ! { $ ($ tts) + } } }
};
}
