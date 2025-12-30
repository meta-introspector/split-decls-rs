// Generated macro for macro_65 (macro)
macro_rules! Depcrate_astmacro_65 {
() => {
// Module: crate::ast
// Provides: {"macro_65"}
// Dependencies: {}
tokenizable ! { # [doc = " Terminators of a [case clause](CaseClause)."] enum ClauseSep { # [doc = " No subsequent matches are attempted after the first pattern match."] Break (";;") , # [doc = " Test the patterns in the next clause, if any, and execute any"] # [doc = " associated command-list on a successful match."] Continue (";;&") , # [doc = " Execute the command-list associated with the next clause, if any."] Fallthrough (";&") , } }
};
}
