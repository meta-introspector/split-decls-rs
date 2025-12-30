// Generated macro for WordSgmt (enum)
macro_rules! Depcrate_wordWordSgmt {
() => {
// Module: crate::word
// Provides: {"WordSgmt"}
// Dependencies: {}
# [derive (Debug , From)] # [decl (enum , name = "WordSgmt" , vis = "pub" , hash = "5d622f2c")] pub enum WordSgmt { # [doc = " Brace expansion."] BraceExpansion (Vec < Word >) , # [doc = " Dollar-prefixed expression."] # [from] DollarExp (DollarExp) , # [from] DoubleQuoted (DoubleQuoted) , # [doc = " Pattern match sequence."] Glob (Spanned < String >) , # [doc = " Literal, unquoted string."] Lit (Spanned < String >) , SingleQuoted (Spanned < String >) , }
};
}
