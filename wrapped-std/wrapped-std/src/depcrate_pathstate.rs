// Generated macro for State (enum)
macro_rules! Depcrate_pathState {
() => {
// Module: crate::path
// Provides: {"State"}
// Dependencies: {}
# [doc = " Component parsing works by a double-ended state machine; the cursors at the"] # [doc = " front and back of the path each keep track of what parts of the path have"] # [doc = " been consumed so far."] # [doc = ""] # [doc = " Going front to back, a path is made up of a prefix, a starting"] # [doc = " directory component, and a body (of normal components)"] # [derive (Copy , Clone , PartialEq , PartialOrd , Debug)] enum State { Prefix = 0 , StartDir = 1 , Body = 2 , Done = 3 , }
};
}
