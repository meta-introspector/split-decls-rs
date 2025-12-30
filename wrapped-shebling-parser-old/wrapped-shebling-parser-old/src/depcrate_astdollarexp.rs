// Generated macro for DollarExp (enum)
macro_rules! Depcrate_astDollarExp {
() => {
// Module: crate::ast
// Provides: {"DollarExp"}
// Dependencies: {}
# [derive (Debug , From , PartialEq)] pub (crate) enum DollarExp { # [doc = " Arithmetic expansion. Can happen inside `$(())` or `$[]`."] Arith (ArithSeq) , # [doc = " `ksh`-style command expansion (e.g. `${ foo; }`)."] CmdExpansion (Term) , # [doc = " Dollar-prefixed command substitution (e.g. `$(foo)`)."] # [doc = ""] # [doc = " The contained [Term] will be [None] iff the command substitution is `$()`."] CmdSub (Option < Term >) , # [doc = " `$\"\"` expression, used for locale-specific translation."] DoubleQuoting (DoubleQuoted) , # [doc = " Shell parameter expansion (e.g. `${!foo[@]}`)."] ParamExpansion (Vec < WordSgmt >) , # [doc = " `$''` expression, used for ANSI-C quoting."] # [from (ignore)] SingleQuoting (String) , # [doc = " A variable."] # [from (ignore)] Var (String) , }
};
}
