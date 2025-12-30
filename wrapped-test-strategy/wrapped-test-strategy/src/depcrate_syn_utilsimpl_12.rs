// Generated macro for impl_12 (impl)
macro_rules! Depcrate_syn_utilsimpl_12 {
() => {
// Module: crate::syn_utils
// Provides: {"impl_12"}
// Dependencies: {}
impl Args { pub fn new () -> Self { Self (Punctuated :: new ()) } pub fn expect_single_value (& self , span : Span) -> Result < & Expr > { if self . len () != 1 { bail ! (span , "expect 1 arguments, but supplied {} arguments." , self . len ()) ; } match & self [0] { Arg :: Value (expr) => Ok (expr) , Arg :: NameValue { .. } => bail ! (span , "expected unnamed argument.") , } } }
};
}
