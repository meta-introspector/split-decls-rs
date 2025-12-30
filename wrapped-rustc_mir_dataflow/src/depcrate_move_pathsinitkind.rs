// Generated macro for InitKind (enum)
macro_rules! Depcrate_move_pathsInitKind {
() => {
// Module: crate::move_paths
// Provides: {"InitKind"}
// Dependencies: {}
# [doc = " Additional information about the initialization."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum InitKind { # [doc = " Deep init, even on panic"] Deep , # [doc = " Only does a shallow init"] Shallow , # [doc = " This doesn't initialize the variable on panic (and a panic is possible)."] NonPanicPathOnly , }
};
}
