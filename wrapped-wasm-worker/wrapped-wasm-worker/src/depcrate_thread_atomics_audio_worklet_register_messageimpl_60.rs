// Generated macro for impl_60 (impl)
macro_rules! Depcrate_thread_atomics_audio_worklet_register_messageimpl_60 {
() => {
// Module: crate::thread::atomics::audio_worklet::register::message
// Provides: {"impl_60"}
// Dependencies: {}
impl HasMessagePortInterface for MessagePort { fn set_onmessage (& self , value : Option < & Function >) { self . set_onmessage (value) ; } fn post_message (& self , message : & JsValue) -> Result < () , JsValue > { self . post_message (message) } fn post_message_with_transfer (& self , message : & JsValue , transfer : & JsValue ,) -> Result < () , JsValue > { self . post_message_with_transferable (message , transfer) } }
};
}
