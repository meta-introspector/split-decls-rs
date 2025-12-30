// Generated macro for Coproc (struct)
macro_rules! Depcrate_astCoproc {
() => {
// Module: crate::ast
// Provides: {"Coproc"}
// Dependencies: {}
# [doc = " [Command](Cmd) preceded by [`coproc`](Keyword::Coproc). A [coprocess](Coproc)"] # [doc = " is executed asynchronously in a subshell."] # [derive (Debug , PartialEq)] pub (crate) struct Coproc { name : Option < String > , cmd : Box < Cmd > , }
};
}
