// Generated macro for spawn_without_message (function)
macro_rules! Depcrate_thread_atomics_spawnspawn_without_message {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"spawn_without_message"}
// Dependencies: {}
# [doc = " Spawn if no message requires transferring through JS."] fn spawn_without_message < T > (thread : Thread , stack_size : Option < usize > , result_receiver : oneshot :: Receiver < T > , # [cfg (feature = "message")] spawn_receiver : channel :: Receiver < SpawnData > , task : Task < '_ > ,) -> JoinHandle < T > { if super :: is_main_thread () { main :: init_main_thread () ; spawn_internal (thread . id () , thread . name () , stack_size , # [cfg (feature = "message")] spawn_receiver , Box :: new (task) ,) ; } else { let task = unsafe { mem :: transmute :: < Task < '_ > , Task < 'static > > (task) } ; Command :: Spawn (SpawnData { id : thread . id () , name : thread . 0 . name . clone () , stack_size , # [cfg (feature = "message")] spawn_receiver , task , }) . send () ; } JoinHandle { receiver : Some (result_receiver) , thread , } }
};
}
