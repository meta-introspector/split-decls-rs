// Generated macro for TaskEntry (enum)
macro_rules! DepcrateTaskEntry {
() => {
// Module: crate
// Provides: {"TaskEntry"}
// Dependencies: {}
enum TaskEntry { # [doc = " Task was added and not yet removed."] Handle (AbortHandle) , # [doc = " Task was removed before it was added. This can happen if a spawned"] # [doc = " future completes before the spawning thread can add it to the map."] Tombstone , }
};
}
