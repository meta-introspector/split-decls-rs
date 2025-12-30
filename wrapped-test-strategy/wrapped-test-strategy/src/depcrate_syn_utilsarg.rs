// Generated macro for Arg (enum)
macro_rules! Depcrate_syn_utilsArg {
() => {
// Module: crate::syn_utils
// Provides: {"Arg"}
// Dependencies: {}
# [derive (ToTokens , Parse)] pub enum Arg { NameValue { # [parse (peek , any)] name : Ident , # [parse (peek)] eq_token : Token ! [=] , value : Expr , } , Value (Expr) , }
};
}
