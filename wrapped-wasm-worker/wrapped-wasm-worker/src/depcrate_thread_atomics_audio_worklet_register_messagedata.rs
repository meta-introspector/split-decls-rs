// Generated macro for Data (struct)
macro_rules! Depcrate_thread_atomics_audio_worklet_register_messageData {
() => {
// Module: crate::thread::atomics::audio_worklet::register::message
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data sent to initialize the audio worklet."] # [derive (Debug)] # [cfg (feature = "message")] pub (super) struct Data { # [doc = " [`Thread`]."] pub (super) thread : Thread , # [doc = " Stack size of the thread."] pub (super) stack_size : Option < usize > , # [doc = " [`Sender`] to send back the associated [`ThreadMemory`]."] pub (super) memory_sender : Sender < ThreadMemory > , }
};
}
