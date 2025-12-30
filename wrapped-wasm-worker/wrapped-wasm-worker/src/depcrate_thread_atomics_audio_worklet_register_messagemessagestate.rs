// Generated macro for MessageState (struct)
macro_rules! Depcrate_thread_atomics_audio_worklet_register_messageMessageState {
() => {
// Module: crate::thread::atomics::audio_worklet::register::message
// Provides: {"MessageState"}
// Dependencies: {}
# [doc = " Message to be sent."] # [derive (Debug)] pub (super) struct MessageState { # [doc = " Values to be [serialized](https://developer.mozilla.org/en-US/docs/Glossary/Serializable_object)."] pub (super) serialize : JsValue , # [doc = " Values to be [transferred](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects)."] pub (super) transfer : Option < Array > , }
};
}
