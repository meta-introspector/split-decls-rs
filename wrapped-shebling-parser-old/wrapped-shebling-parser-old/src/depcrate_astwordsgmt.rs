// Generated macro for WordSgmt (enum)
macro_rules! Depcrate_astWordSgmt {
() => {
// Module: crate::ast
// Provides: {"WordSgmt"}
// Dependencies: {}
# [derive (Debug , From , PartialEq)] pub (crate) enum WordSgmt { # [doc = " Backquoted command substitution."] BackQuoted (Term) , # [doc = " Brace expansion."] BraceExpansion (Vec < Word >) , # [doc = " Dollar-prefixed expression."] # [from] DollarExp (DollarExp) , # [doc = " Double-quoted string."] # [from] DoubleQuoted (DoubleQuoted) , # [doc = " Pattern match sequence."] Glob (String) , # [doc = " Literal, unquoted string."] Lit (String) , # [doc = " Process substitution."] ProcSub (Vec < Term >) , # [doc = " Single-quoted string."] SingleQuoted (String) , }
};
}
