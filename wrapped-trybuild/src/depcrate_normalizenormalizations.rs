// Generated macro for normalizations (macro)
macro_rules! Depcrate_normalizenormalizations {
() => {
// Module: crate::normalize
// Provides: {"normalizations"}
// Dependencies: {}
macro_rules ! normalizations { ($ ($ name : ident ,) *) => { # [derive (PartialOrd , PartialEq , Copy , Clone)] enum Normalization { $ ($ name ,) * } impl Normalization { const ALL : &'static [Self] = & [$ ($ name) ,*] ; } impl Default for Variations { fn default () -> Self { Variations { variations : [$ (($ name , String :: new ()) . 1) ,*] , } } } } ; }
};
}
