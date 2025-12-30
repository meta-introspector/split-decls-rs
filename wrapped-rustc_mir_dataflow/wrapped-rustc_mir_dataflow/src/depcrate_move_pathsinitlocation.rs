// Generated macro for InitLocation (enum)
macro_rules! Depcrate_move_pathsInitLocation {
() => {
// Module: crate::move_paths
// Provides: {"InitLocation"}
// Dependencies: {}
# [doc = " Initializations can be from an argument or from a statement. Arguments"] # [doc = " do not have locations, in those cases the `Local` is kept.."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum InitLocation { Argument (Local) , Statement (Location) , }
};
}
