// Generated macro for SimpleCmd (struct)
macro_rules! Depcrate_astSimpleCmd {
() => {
// Module: crate::ast
// Provides: {"SimpleCmd"}
// Dependencies: {}
# [doc = " Sequence of [Word]s separated by blanks, terminated by a"] # [doc = " [control operator](ControlOp)."] # [derive (Debug , PartialEq)] pub (crate) struct SimpleCmd { cmd : Option < Word > , prefix : Vec < CmdPrefixSgmt > , suffix : Vec < CmdSuffixSgmt > , }
};
}
