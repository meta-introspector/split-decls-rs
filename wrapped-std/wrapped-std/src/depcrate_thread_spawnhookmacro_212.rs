// Generated macro for macro_212 (macro)
macro_rules! Depcrate_thread_spawnhookmacro_212 {
() => {
// Module: crate::thread::spawnhook
// Provides: {"macro_212"}
// Dependencies: {}
crate :: thread_local ! { # [doc = " A thread local linked list of spawn hooks."] # [doc = ""] # [doc = " It is a linked list of Arcs, such that it can very cheaply be inherited by spawned threads."] # [doc = ""] # [doc = " (That technically makes it a set of linked lists with shared tails, so a linked tree.)"] static SPAWN_HOOKS : Cell < SpawnHooks > = const { Cell :: new (SpawnHooks { first : None }) } ; }
};
}
