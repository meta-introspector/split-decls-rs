// Generated macro for Construct (enum)
macro_rules! Depcrate_astConstruct {
() => {
// Module: crate::ast
// Provides: {"Construct"}
// Dependencies: {}
# [doc = " A shell programming language construct, which form the body of a"] # [doc = " [compound command](CompoundCmd)."] # [derive (Debug , From , PartialEq)] pub (crate) enum Construct { # [from] ArithSeq (ArithSeq) , BatsTest (Function) , BraceGroup (Term) , # [from] Case (CaseCmd) , # [from] Cond (Cond) , # [from] ForLoop (ForLoop) , Function (Function) , # [from] If (IfCmd) , Select (InListed) , Subshell (Term) , Until (CondBlock) , While (CondBlock) , }
};
}
