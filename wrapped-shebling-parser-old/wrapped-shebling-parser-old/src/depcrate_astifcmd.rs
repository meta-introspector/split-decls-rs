// Generated macro for IfCmd (struct)
macro_rules! Depcrate_astIfCmd {
() => {
// Module: crate::ast
// Provides: {"IfCmd"}
// Dependencies: {}
# [doc = " Conditional command construct beginning with [`if`](Keyword::If)."] # [derive (Debug , PartialEq)] pub (crate) struct IfCmd { if_branch : CondBlock , elif_branches : Vec < CondBlock > , else_term : Option < Term > , }
};
}
