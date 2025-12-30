// Generated macro for DESTROY_SENDER (static)
macro_rules! Depcrate_thread_atomics_audio_worklet_mainDESTROY_SENDER {
() => {
// Module: crate::thread::atomics::audio_worklet::main
// Provides: {"DESTROY_SENDER"}
// Dependencies: {}
# [doc = " [`ThreadId`] to destroy [`Sender`] to the main thread."] # [allow (clippy :: disallowed_methods , reason = "this is guaranteed to be initialized from the main thread before any other thread \
	          will try to access it")] pub (super) static DESTROY_SENDER : OnceLock < Sender < ThreadId > > = OnceLock :: new () ;
};
}
