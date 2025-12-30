// Generated macro for macro_220 (macro)
macro_rules! Depcrate_thread_atomics_spawn_messagemacro_220 {
() => {
// Module: crate::thread::atomics::spawn::message
// Provides: {"macro_220"}
// Dependencies: {}
thread_local ! { pub (in super :: super) static SPAWN_SENDER : RefCell < Option < channel :: Sender < SpawnData >>> = const { RefCell :: new (None) } ; }
};
}
