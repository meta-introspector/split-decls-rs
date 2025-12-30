// Generated macro for DollarExp (enum)
macro_rules! Depcrate_expansionDollarExp {
() => {
// Module: crate::expansion
// Provides: {"DollarExp"}
// Dependencies: {}
# [doc = " Shell expansions."] # [derive (Debug , From)] # [decl (enum , name = "DollarExp" , vis = "pub" , hash = "5ad2939b")] pub enum DollarExp { # [doc = " Arithmetic expansion. Can happen inside `$(())` or `$[]`."] Arith (ArithSeq) , # [doc = " `ksh`-style command expansion (e.g. `${ foo; }`)."] # [doc = " Dollar-prefixed command substitution (e.g. `$(foo)`)."] # [doc = ""] # [doc = " The contained [Term] will be [None] iff the command substitution is `$()`."] # [doc = " `$\"\"` expression, used for locale-specific translation."] DoubleQuoting (DoubleQuoted) , # [doc = " Shell parameter expansion (e.g. `${!foo[@]}`)."] ParamExpansion (Vec < WordSgmt >) , # [doc = " `$''` expression, used for ANSI-C quoting."] # [from (ignore)] SingleQuoting (Spanned < String >) , # [doc = " A variable."] # [from (ignore)] Var (Spanned < String >) , }
};
}
