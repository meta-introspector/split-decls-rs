// Generated macro for HasMessagePortInterface (trait)
macro_rules! Depcrate_thread_atomics_spawn_messageHasMessagePortInterface {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"HasMessagePortInterface"}
// Dependencies: {}
# [doc = " Trait over any type having an interface like"] # [doc = " [`MessagePort`](web_sys::MessagePort)."] pub (in super :: super) trait HasMessagePortInterface { # [doc = " Setter for the [`message`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/message_event) event handler."] fn set_onmessage (& self , value : Option < & Function >) ; # [doc = " [`MessagePort.postMessage()`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)."] fn post_message (& self , message : & JsValue) -> Result < () , JsValue > ; # [doc = " [`MessagePort.postMessage()`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)."] fn post_message_with_transfer (& self , message : & JsValue , transfer : & JsValue ,) -> Result < () , JsValue > ; }
};
}
