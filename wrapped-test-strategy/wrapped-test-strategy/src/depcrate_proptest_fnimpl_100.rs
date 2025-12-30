// Generated macro for impl_100 (impl)
macro_rules! Depcrate_proptest_fnimpl_100 {
() => {
// Module: crate::proptest_fn
// Provides: {"impl_100"}
// Dependencies: {}
impl Async { fn apply (& self , block : & Block , output : ReturnType) -> TokenStream { let body ; let output_type ; let ret_expr ; match output { ReturnType :: Default => { body = quote ! { # block Ok (()) } ; output_type = quote ! (:: core :: result :: Result < _ , :: proptest :: test_runner :: TestCaseError >) ; ret_expr = quote ! { ret ? } ; } ReturnType :: Type (_ , ty) => { body = quote ! { # block } ; output_type = quote ! (# ty) ; ret_expr = quote ! { :: std :: result :: Result :: map_err (ret , | e | :: proptest :: test_runner :: TestCaseError :: fail (:: std :: string :: ToString :: to_string (& e))) ? } ; } } match self { Async :: Tokio => { quote ! { let ret : # output_type = tokio :: runtime :: Runtime :: new () . unwrap () . block_on (async move { # body }) ; # ret_expr ; } } Async :: Expr (expr) => { quote ! { let ret : # output_type = (# expr) (async move { # body }) ; # ret_expr ; } } } } }
};
}
