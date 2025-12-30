// Generated macro for Redir (struct)
macro_rules! Depcrate_astRedir {
() => {
// Module: crate::ast
// Provides: {"Redir"}
// Dependencies: {}
# [doc = " Shell redirection, used to change the files a [command](Cmd) reads"] # [doc = " and writes to."] # [derive (Debug , PartialEq)] pub (crate) struct Redir { file_desc : Option < FileDesc > , op : RedirOp , word : Word , }
};
}
