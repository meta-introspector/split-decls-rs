// Generated macro for send_message (function)
macro_rules! Depcrate_thread_atomics_spawn_messagesend_message {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"send_message"}
// Dependencies: {}
# [doc = " Send [`MessageSend`] over any [`HasMessagePortInterface`]."] fn send_message (port : & impl HasMessagePortInterface , serialize : & JsValue , transfer : Option < Array > ,) -> io :: Result < () > { let result = if let Some (transfer) = transfer { port . post_message_with_transfer (& Array :: of2 (serialize , & transfer) , & transfer) } else { port . post_message (& Array :: of1 (serialize)) } ; if let Err (error) = result { port . post_message (& JsValue :: UNDEFINED) . expect ("`DedicatedWorkerGlobalScope.postMessage()` is not expected to fail without a \
			 `transfer` object" ,) ; Err (super :: super :: error_from_exception (error)) } else { Ok (()) } }
};
}
