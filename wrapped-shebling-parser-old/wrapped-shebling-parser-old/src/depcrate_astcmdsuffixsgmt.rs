// Generated macro for CmdSuffixSgmt (enum)
macro_rules! Depcrate_astCmdSuffixSgmt {
() => {
// Module: crate::ast
// Provides: {"CmdSuffixSgmt"}
// Dependencies: {}
# [doc = " A [Word] that follows the command in a [simple command](SimpleCmd)."] # [derive (Debug , From , PartialEq)] pub (crate) enum CmdSuffixSgmt { ArithSeq (ArithSeq) , Assign (Assign) , Pipeline (Pipeline) , Redir (Redir) , Word (Word) , }
};
}
