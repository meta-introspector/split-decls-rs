// Generated macro for spawn_internal (function)
macro_rules! Depcrate_thread_atomics_spawn_messagespawn_internal {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"spawn_internal"}
// Dependencies: {}
# [doc = " Spawning thread regardless of being nested."] fn spawn_internal (id : ThreadId , name : Option < & str > , stack_size : Option < usize > , spawn_receiver : Receiver < SpawnData > , serialize : & JsValue , transfer : Option < Array > , task : Task < '_ > ,) -> io :: Result < () > { let result = super :: spawn_common (id , name , spawn_receiver , task , # [cfg (not (feature = "audio-worklet"))] | worker : & Worker , module , memory , task | { if let Some (transfer) = transfer { worker . post_message_with_transfer (& Array :: of5 (module , memory , & stack_size . into () , & task , serialize) , & transfer ,) } else { worker . post_message (& Array :: of5 (module , memory , & stack_size . into () , & task , serialize ,)) } } , # [cfg (feature = "audio-worklet")] | worker : & Worker , module , memory , task | { THREAD_LOCK_INDEXES . with (| indexes | { if let Some (transfer) = transfer { worker . post_message_with_transfer (& ArrayExt :: of6 (module , memory , & stack_size . into () , indexes , & task , serialize ,) , & transfer ,) } else { worker . post_message (& ArrayExt :: of6 (module , memory , & stack_size . into () , indexes , & task , serialize ,)) } }) } ,) ; if let Err (error) = result { Err (super :: super :: error_from_exception (error)) } else { Ok (()) } }
};
}
