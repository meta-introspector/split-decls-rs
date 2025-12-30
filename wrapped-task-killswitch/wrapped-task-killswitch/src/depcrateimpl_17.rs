// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl ActiveTasks { fn kill_all (& self) { self . tasks . retain (| _ , entry | { if let TaskEntry :: Handle (task) = entry { task . abort () ; } false }) ; } fn add_task_if (& self , handle : AbortHandle , cond : impl FnOnce () -> bool ,) -> Result < () , AbortHandle > { use dashmap :: Entry :: * ; let id = handle . id () ; match self . tasks . entry (id) { Vacant (e) => { if ! cond () { return Err (handle) ; } e . insert (TaskEntry :: Handle (handle)) ; } , Occupied (e) if matches ! (e . get () , TaskEntry :: Tombstone) => { e . remove () ; } , Occupied (_) => panic ! ("tokio task ID already in use: {id}") , } Ok (()) } fn remove_task (& self , id : task :: Id) { use dashmap :: Entry :: * ; match self . tasks . entry (id) { Vacant (e) => { e . insert (TaskEntry :: Tombstone) ; } , Occupied (e) if matches ! (e . get () , TaskEntry :: Tombstone) => { } , Occupied (e) => { e . remove () ; } , } } }
};
}
