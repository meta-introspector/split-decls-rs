// Generated macro for broadcast_after_spawn_broadcast (function)
macro_rules! Depcrate_broadcast_testsbroadcast_after_spawn_broadcast {
() => {
// Module: crate::broadcast::tests
// Provides: {"broadcast_after_spawn_broadcast"}
// Dependencies: {}
# [test] fn broadcast_after_spawn_broadcast () { let (tx , rx) = channel () ; crate :: spawn_broadcast (move | ctx | tx . send (ctx . index ()) . unwrap ()) ; crate :: broadcast (| _ | { }) ; let mut v : Vec < _ > = rx . try_iter () . collect () ; v . sort_unstable () ; assert ! (v . into_iter () . eq (0 .. crate :: current_num_threads ())) ; }
};
}
