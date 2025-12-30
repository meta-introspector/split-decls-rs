// Generated macro for Cmd (enum)
macro_rules! Depcrate_astCmd {
() => {
// Module: crate::ast
// Provides: {"Cmd"}
// Dependencies: {}
# [doc = " A sequence of words defining a shell command."] # [derive (Debug , From , PartialEq)] pub (crate) enum Cmd { Compound (CompoundCmd) , Coproc (Coproc) , Simple (SimpleCmd) , }
};
}
