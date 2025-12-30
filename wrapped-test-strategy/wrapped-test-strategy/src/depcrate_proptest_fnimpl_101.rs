// Generated macro for impl_101 (impl)
macro_rules! Depcrate_proptest_fnimpl_101 {
() => {
// Module: crate::proptest_fn
// Provides: {"impl_101"}
// Dependencies: {}
impl syn :: parse :: Parse for Async { fn parse (input : syn :: parse :: ParseStream) -> Result < Self > { if input . peek (LitStr) { let s : LitStr = input . parse () ? ; match s . value () . as_str () { "tokio" => Ok (Async :: Tokio) , _ => bail ! (s . span () , "expected `tokio`.") , } } else { Ok (Async :: Expr (input . parse () ?)) } } }
};
}
