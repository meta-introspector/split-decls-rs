// Generated macro for COMMAND_SENDER (static)
macro_rules! Depcrate_thread_atomics_mainCOMMAND_SENDER {
() => {
// Module: crate::thread::atomics::main
// Provides: {"COMMAND_SENDER"}
// Dependencies: {}
# [doc = " [`Command`] [`Sender`] to the main thread."] # [allow (clippy :: disallowed_methods , reason = "this is guaranteed to be initialized from the main thread before any other thread \
	          will try to access it")] static COMMAND_SENDER : OnceLock < Sender < Command > > = OnceLock :: new () ;
};
}
