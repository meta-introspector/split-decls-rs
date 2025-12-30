// Generated macro for register (function)
macro_rules! Depcrate_low_level_piperegister {
() => {
// Module: crate::low_level::pipe
// Provides: {"register"}
// Dependencies: {}
# [doc = " Registers a write to a self-pipe whenever there's the signal."] # [doc = ""] # [doc = " The ownership of pipe is taken and will be closed whenever the created action is unregistered."] # [doc = ""] # [doc = " Note that if you want to register the same pipe for multiple signals, there's `try_clone`"] # [doc = " method on many unix socket primitives."] # [doc = ""] # [doc = " See [`register_raw`] for further details."] pub fn register < P > (signal : c_int , pipe : P) -> Result < SigId , Error > where P : IntoRawFd + 'static , { register_raw (signal , pipe . into_raw_fd ()) }
};
}
