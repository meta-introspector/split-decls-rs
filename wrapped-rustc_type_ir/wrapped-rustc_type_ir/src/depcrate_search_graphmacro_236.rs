// Generated macro for macro_236 (macro)
macro_rules! Depcrate_search_graphmacro_236 {
() => {
// Module: crate::search_graph
// Provides: {"macro_236"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " Tracks how nested goals have been accessed. This is necessary to disable"] # [doc = " global cache entries if computing them would otherwise result in a cycle or"] # [doc = " access a provisional cache entry."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct PathsToNested : u8 { # [doc = " The initial value when adding a goal to its own nested goals."] const EMPTY = 1 << 0 ; const INDUCTIVE = 1 << 1 ; const UNKNOWN = 1 << 2 ; const COINDUCTIVE = 1 << 3 ; const FORCED_AMBIGUITY = 1 << 4 ; } }
};
}
