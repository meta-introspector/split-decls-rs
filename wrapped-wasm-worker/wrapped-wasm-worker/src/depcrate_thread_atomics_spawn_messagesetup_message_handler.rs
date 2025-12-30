// Generated macro for setup_message_handler (function)
macro_rules! Depcrate_thread_atomics_spawn_messagesetup_message_handler {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"setup_message_handler"}
// Dependencies: {}
# [doc = " Setup `message` event handler."] pub (in super :: super) fn setup_message_handler (this : & impl HasMessagePortInterface , spawn_receiver : Receiver < SpawnData > ,) -> Closure < dyn Fn (MessageEvent) > { let message_handler = Closure :: new (move | event : MessageEvent | { let data = spawn_receiver . try_recv () . expect ("expected data to have been sent before message") ; let message = event . data () ; if message . is_undefined () { return ; } let mut values = message . unchecked_into :: < Array > () . into_iter () ; let serialize = values . next () . expect ("no serialized data found") ; let transfer = values . next () . map (Array :: unchecked_from_js) ; spawn_internal (data . id , data . name . as_deref () , data . stack_size , data . spawn_receiver , & serialize , transfer , Box :: new (data . task) ,) . expect ("unexpected serialization error when serialization succeeded when sending this") ; }) ; this . set_onmessage (Some (message_handler . as_ref () . unchecked_ref ())) ; message_handler }
};
}
