// Generated macro for impl_225 (impl)
macro_rules! Depcrate_thread_atomics_spawn_messageimpl_225 {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"impl_225"}
// Dependencies: {}
impl HasMessagePortInterface for Worker { fn set_onmessage (& self , value : Option < & Function >) { self . set_onmessage (value) ; } fn post_message (& self , message : & JsValue) -> Result < () , JsValue > { self . post_message (message) } fn post_message_with_transfer (& self , message : & JsValue , transfer : & JsValue ,) -> Result < () , JsValue > { self . post_message_with_transfer (message , transfer) } }
};
}
