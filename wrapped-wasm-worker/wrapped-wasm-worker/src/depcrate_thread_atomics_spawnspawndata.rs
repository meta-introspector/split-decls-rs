// Generated macro for SpawnData (struct)
macro_rules! Depcrate_thread_atomics_spawnSpawnData {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"SpawnData"}
// Dependencies: {}
# [doc = " Data to spawn new thread."] pub (super) struct SpawnData { # [doc = " [`ThreadId`] of the thread to be spawned."] pub (super) id : ThreadId , # [doc = " Name of the thread."] pub (super) name : Option < String > , # [doc = " Stack size of the thread."] pub (super) stack_size : Option < usize > , # [doc = " [`Task`]s with messages to spawn."] # [cfg (feature = "message")] pub (super) spawn_receiver : channel :: Receiver < SpawnData > , # [doc = " Task."] pub (super) task : Task < 'static > , }
};
}
